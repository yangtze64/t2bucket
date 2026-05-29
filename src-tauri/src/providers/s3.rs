use super::{ObjectItem, StorageProvider};
use hmac::{Hmac, Mac};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct S3Provider {
    access_key: String,
    secret_key: String,
    region: String,
}

impl S3Provider {
    pub fn new(access_key: &str, secret_key: &str, region: &str) -> Self {
        Self {
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            region: region.to_string(),
        }
    }

    fn is_china_region(&self) -> bool {
        self.region.starts_with("cn-")
    }

    fn list_buckets_host(&self) -> String {
        if self.is_china_region() {
            format!("s3.{}.amazonaws.com.cn", self.region)
        } else {
            "s3.amazonaws.com".to_string()
        }
    }

    fn bucket_host(&self, bucket: &str) -> String {
        if self.is_china_region() {
            format!("{}.s3.{}.amazonaws.com.cn", bucket, self.region)
        } else {
            format!("{}.s3.{}.amazonaws.com", bucket, self.region)
        }
    }

    fn client() -> Client {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("failed to build HTTP client")
    }

    fn get_date_string() -> String {
        chrono::Utc::now().format("%Y%m%dT%H%M%SZ").to_string()
    }

    fn sign(&self, method: &str, host: &str, path: &str, params: &HashMap<&str, &str>, payload_hash: &str) -> (String, String, String) {
        let date_str = Self::get_date_string();
        let date_short = &date_str[0..8];

        let mut sorted_params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, *v)).collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));
        let canonical_query_string: String = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let mut all_headers: Vec<(&str, String)> = vec![
            ("host", host.to_string()),
            ("x-amz-content-sha256", payload_hash.to_string()),
            ("x-amz-date", date_str.clone()),
        ];
        all_headers.sort_by(|a, b| a.0.cmp(b.0));

        let canonical_headers: String = all_headers
            .iter()
            .map(|(k, v)| format!("{}:{}\n", k.to_lowercase(), v.trim()))
            .collect::<Vec<String>>()
            .join("");

        let signed_headers: String = all_headers
            .iter()
            .map(|(k, _)| k.to_lowercase())
            .collect::<Vec<String>>()
            .join(";");

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method.to_uppercase(),
            path,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            payload_hash
        );

        let scope = format!("{}/{}/s3/aws4_request", date_short, self.region);
        let string_to_sign = format!(
            "AWS4-HMAC-SHA256\n{}\n{}\n{}",
            date_str,
            scope,
            hex::encode(Sha256::digest(canonical_request.as_bytes()))
        );

        let k_date = hmac_sha256(format!("AWS4{}", self.secret_key).as_bytes(), date_short);
        let k_region = hmac_sha256(&k_date, &self.region);
        let k_service = hmac_sha256(&k_region, "s3");
        let k_signing = hmac_sha256(&k_service, "aws4_request");
        let signature = hex::encode(hmac_sha256(&k_signing, &string_to_sign));

        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
            self.access_key,
            scope,
            signed_headers,
            signature
        );

        (authorization, date_str, payload_hash.to_string())
    }
}

impl StorageProvider for S3Provider {
    fn list_buckets(&self) -> Result<Vec<String>, String> {
        let host = self.list_buckets_host();
        let path = "/";
        let url = format!("https://{}", host);
        let payload_hash = "UNSIGNED-PAYLOAD";
        let (auth, amz_date, _) = self.sign("GET", &host, path, &HashMap::new(), payload_hash);

        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", payload_hash)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let body = resp.text().map_err(|e| format!("读取响应失败: {}", e))?;
        if !status.is_success() {
            return Err(format!("S3 API HTTP {}: {}", status.as_u16(), body));
        }

        Ok(extract_all_tags(&body, "Name"))
    }

    fn list_objects(&self, bucket: &str, prefix: &str, delimiter: &str) -> Result<(Vec<String>, Vec<ObjectItem>), String> {
        let host = self.bucket_host(bucket);
        let path = "/";
        let mut params = HashMap::new();
        params.insert("list-type", "2");
        if !delimiter.is_empty() {
            params.insert("delimiter", delimiter);
        }
        if !prefix.is_empty() {
            params.insert("prefix", prefix);
        }

        let mut url = format!("https://{}", host);
        let query_string: Vec<String> = params.iter().map(|(k, v)| format!("{}={}", k, urlencoding::encode(v))).collect();
        if !query_string.is_empty() {
            url.push('?');
            url.push_str(&query_string.join("&"));
        }

        let payload_hash = "UNSIGNED-PAYLOAD";
        let (auth, amz_date, _) = self.sign("GET", &host, path, &params, payload_hash);

        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", payload_hash)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let body = resp.text().map_err(|e| format!("读取响应失败: {}", e))?;
        if !status.is_success() {
            return Err(format!("S3 API HTTP {}: {}", status.as_u16(), body));
        }

        let prefixes = parse_xml_list(&body, "CommonPrefixes", "Prefix");
        let s3_objects = parse_s3_objects(&body);
        let objects = s3_objects.into_iter().map(|o| ObjectItem {
            key: o.key,
            size: o.size,
            last_modified: o.last_modified,
            is_dir: false,
        }).collect();

        Ok((prefixes, objects))
    }

    fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        let host = self.bucket_host(bucket);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let payload_hash = "UNSIGNED-PAYLOAD";
        let (auth, amz_date, _) = self.sign("GET", &host, &path, &HashMap::new(), payload_hash);

        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", payload_hash)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let err_body = resp.text().unwrap_or_default();
            return Err(format!("S3 API HTTP {}: {}", status.as_u16(), err_body));
        }

        resp.bytes()
            .map(|b| b.to_vec())
            .map_err(|e| format!("读取内容失败: {}", e))
    }

    fn put_object(&self, bucket: &str, key: &str, content: &[u8]) -> Result<(), String> {
        let host = self.bucket_host(bucket);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let payload_hash = hex::encode(Sha256::digest(content));
        let (auth, amz_date, _) = self.sign("PUT", &host, &path, &HashMap::new(), &payload_hash);

        let resp = Self::client()
            .put(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", &payload_hash)
            .body(content.to_vec())
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let err_body = resp.text().unwrap_or_default();
            return Err(format!("上传失败 HTTP {}: {}", status.as_u16(), err_body));
        }
        Ok(())
    }

    fn delete_object(&self, bucket: &str, key: &str) -> Result<(), String> {
        let host = self.bucket_host(bucket);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let payload_hash = "UNSIGNED-PAYLOAD";
        let (auth, amz_date, _) = self.sign("DELETE", &host, &path, &HashMap::new(), payload_hash);

        let resp = Self::client()
            .delete(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .header("x-amz-date", &amz_date)
            .header("x-amz-content-sha256", payload_hash)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let err_body = resp.text().unwrap_or_default();
            return Err(format!("删除失败 HTTP {}: {}", status.as_u16(), err_body));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct S3Object {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}

fn url_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

fn hmac_sha256(key: &[u8], data: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC key");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn parse_xml_list(xml: &str, parent: &str, child: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut in_parent = false;
    for line in xml.lines() {
        let trimmed = line.trim();
        if trimmed == format!("<{}>", parent) {
            in_parent = true;
            continue;
        }
        if trimmed == format!("</{}>", parent) {
            in_parent = false;
            continue;
        }
        if in_parent && trimmed.starts_with(&format!("<{}>", child)) {
            if let Some(val) = extract_text(trimmed, child) {
                results.push(val);
            }
        }
    }
    results
}

fn extract_all_tags(xml: &str, tag: &str) -> Vec<String> {
    let mut results = Vec::new();
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    let mut pos = 0;
    while let Some(start) = xml[pos..].find(&open_tag) {
        let start = pos + start + open_tag.len();
        if let Some(end) = xml[start..].find(&close_tag) {
            results.push(xml[start..start + end].to_string());
            pos = start + end + close_tag.len();
        } else {
            break;
        }
    }
    results
}

fn extract_text(s: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    if s.starts_with(&open_tag) && s.ends_with(&close_tag) {
        Some(s[open_tag.len()..s.len() - close_tag.len()].to_string())
    } else {
        None
    }
}

fn parse_s3_objects(xml: &str) -> Vec<S3Object> {
    let mut results = Vec::new();
    let mut current_key = None;
    let mut current_size = None;
    let mut current_last_modified = None;
    let mut in_contents = false;
    for line in xml.lines() {
        let trimmed = line.trim();
        if trimmed == "<Contents>" {
            in_contents = true;
            continue;
        }
        if trimmed == "</Contents>" {
            if let (Some(key), Some(size), Some(last_modified)) =
                (current_key.take(), current_size.take(), current_last_modified.take())
            {
                results.push(S3Object {
                    key,
                    size,
                    last_modified,
                });
            }
            in_contents = false;
            continue;
        }
        if in_contents {
            if trimmed.starts_with("<Key>") {
                current_key = extract_text(trimmed, "Key");
            } else if trimmed.starts_with("<Size>") {
                if let Some(s) = extract_text(trimmed, "Size") {
                    current_size = s.parse().ok();
                }
            } else if trimmed.starts_with("<LastModified>") {
                current_last_modified = extract_text(trimmed, "LastModified");
            }
        }
    }
    results
}
