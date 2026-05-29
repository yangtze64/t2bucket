use super::{ObjectItem, StorageProvider};
use hmac::Hmac;
use hmac::Mac;
use reqwest::blocking::Client;
use sha1::Sha1;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

type HmacSha1 = Hmac<Sha1>;

#[derive(Debug, Clone)]
pub struct CosProvider {
    secret_id: String,
    secret_key: String,
    region: String,
}

impl CosProvider {
    pub fn new(secret_id: &str, secret_key: &str, region: &str) -> Self {
        Self {
            secret_id: secret_id.to_string(),
            secret_key: secret_key.to_string(),
            region: region.to_string(),
        }
    }

    fn client() -> Client {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("failed to build HTTP client")
    }
}

impl StorageProvider for CosProvider {
    fn list_buckets(&self) -> Result<Vec<String>, String> {
        let host = "service.cos.myqcloud.com";
        let path = "/";
        let url = format!("https://{}/", host);
        let auth = self.sign("get", path, &[], &[("host", host)]);

        eprintln!("COS_DEBUG list_buckets url: {}", url);
        eprintln!("COS_DEBUG list_buckets region: {}", self.region);
        
        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", host)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        let body = resp.text().map_err(|e| format!("读取响应失败: {}", e))?;

        if !status.is_success() {
            return Err(format!("COS API HTTP {}: {}", status.as_u16(), body));
        }
        let buckets = extract_all_tags(&body, "Name");
        Ok(buckets)
    }

    fn list_objects(&self, bucket: &str, prefix: &str, delimiter: &str) -> Result<(Vec<String>, Vec<ObjectItem>), String> {
        let host = format!("{}.cos.{}.myqcloud.com", bucket, self.region);
        let path = "/";
        let params = vec![
            ("delimiter", delimiter),
            ("prefix", prefix),
        ];
        let url = format!("https://{}?delimiter={}&prefix={}", host, urlencoding::encode(delimiter), urlencoding::encode(prefix));
        let auth = self.sign("get", path, &params, &[("host", &host)]);

        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let body = resp.text().map_err(|e| format!("读取响应失败: {}", e))?;

        let prefixes = parse_xml_list(&body, "CommonPrefixes", "Prefix");
        let cos_objects = parse_cos_objects(&body);
        let objects = cos_objects.into_iter().map(|o| ObjectItem {
            key: o.key,
            size: o.size,
            last_modified: o.last_modified,
            is_dir: false,
        }).collect();

        Ok((prefixes, objects))
    }

    fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        let host = format!("{}.cos.{}.myqcloud.com", bucket, self.region);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let auth = self.sign("get", &path, &[], &[("host", &host)]);

        eprintln!("COS_DEBUG get_object 正在下载: bucket={}, key={}", bucket, key);
        eprintln!("COS_DEBUG get_object url: {}", url);

        let resp = Self::client()
            .get(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        eprintln!("COS_DEBUG get_object status: {}", status);
        if !status.is_success() {
            let err_body = resp.text().unwrap_or_default();
            eprintln!("COS_DEBUG get_object error: {}", err_body);
            return Err(format!("COS API HTTP {}: {}", status.as_u16(), err_body));
        }

        let bytes = resp.bytes()
            .map(|b| b.to_vec())
            .map_err(|e| format!("读取内容失败: {}", e))?;

        eprintln!("COS_DEBUG get_object 下载成功, {} bytes", bytes.len());
        Ok(bytes)
    }

    fn put_object(&self, bucket: &str, key: &str, content: &[u8]) -> Result<(), String> {
        let host = format!("{}.cos.{}.myqcloud.com", bucket, self.region);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let auth = self.sign("put", &path, &[], &[("host", &host)]);

        let resp = Self::client()
            .put(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .body(content.to_vec())
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(format!("上传失败 HTTP {}", status.as_u16()));
        }
        Ok(())
    }

    fn delete_object(&self, bucket: &str, key: &str) -> Result<(), String> {
        let host = format!("{}.cos.{}.myqcloud.com", bucket, self.region);
        let path = format!("/{}", key);
        let url = format!("https://{}/{}", host, key);
        let auth = self.sign("delete", &path, &[], &[("host", &host)]);

        let resp = Self::client()
            .delete(&url)
            .header("Authorization", &auth)
            .header("Host", &host)
            .send()
            .map_err(|e| format!("请求失败: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(format!("删除失败 HTTP {}", status.as_u16()));
        }
        Ok(())
    }
}

impl CosProvider {
    fn sign(&self, method: &str, path: &str, params: &[(&str, &str)], headers: &[(&str, &str)]) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let key_time = format!("{};{}", now - 60, now + 3600);

        // 1. 生成 SignKey: HMAC-SHA1(SecretKey, KeyTime) -> 16进制小写
        let sign_key = hex::encode(hmac_sha1(&self.secret_key, &key_time));

        // 2. 处理 URL 参数
        let mut sorted_params: Vec<(String, String)> = params
            .iter()
            .map(|(k, v)| (
                urlencoding::encode(k).to_lowercase(),
                urlencoding::encode(v).to_string(),
            ))
            .collect();
        sorted_params.sort_by(|a, b| a.0.cmp(&b.0));
        
        let url_param_list = sorted_params
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
            .join(";");
        
        let http_parameters = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        // 3. 处理 Headers
        let mut sorted_headers: Vec<(String, String)> = headers
            .iter()
            .map(|(k, v)| (
                urlencoding::encode(k).to_lowercase(),
                urlencoding::encode(v).to_string(),
            ))
            .collect();
        sorted_headers.sort_by(|a, b| a.0.cmp(&b.0));
        
        let header_list = sorted_headers
            .iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
            .join(";");
        
        let http_headers = sorted_headers
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        // 4. 生成 HttpString
        let http_string = format!(
            "{}\n{}\n{}\n{}\n",
            method.to_lowercase(),
            path,
            http_parameters,
            http_headers
        );
        
        // 5. 生成 StringToSign
        let sha1_http_string = hex::encode(sha1_hash(http_string.as_bytes()));
        let string_to_sign = format!("sha1\n{}\n{}\n", key_time, sha1_http_string);
        
        // 6. 生成 Signature: HMAC-SHA1(SignKey, StringToSign) -> 16进制小写
        let signature = hex::encode(hmac_sha1(&sign_key, &string_to_sign));
        
        // 7. 组装最终签名
        format!(
            "q-sign-algorithm=sha1&q-ak={}&q-sign-time={}&q-key-time={}&q-header-list={}&q-url-param-list={}&q-signature={}",
            self.secret_id,
            key_time,
            key_time,
            header_list,
            url_param_list,
            signature
        )
    }
}

#[derive(Debug)]
struct CosObject {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}

fn sha1_hash(data: &[u8]) -> Vec<u8> {
    use sha1::Digest;
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn hmac_sha1(key: &str, data: &str) -> Vec<u8> {
    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("HMAC key");
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

fn parse_cos_objects(xml: &str) -> Vec<CosObject> {
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
                results.push(CosObject {
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
