mod cos;
mod s3;

pub use cos::CosProvider;
pub use s3::S3Provider;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectItem {
    pub key: String,
    pub size: u64,
    pub last_modified: String,
    pub is_dir: bool,
}

pub trait StorageProvider: Send + Sync {
    fn list_buckets(&self) -> Result<Vec<String>, String>;
    fn list_objects(&self, bucket: &str, prefix: &str, delimiter: &str) -> Result<(Vec<String>, Vec<ObjectItem>), String>;
    fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String>;
    fn put_object(&self, bucket: &str, key: &str, content: &[u8]) -> Result<(), String>;
    fn delete_object(&self, bucket: &str, key: &str) -> Result<(), String>;
}

pub fn create_provider(
    provider: &str,
    secret_id: &str,
    secret_key: &str,
    region: &str,
) -> Box<dyn StorageProvider> {
    match provider.to_lowercase().as_str() {
        "cos" => Box::new(CosProvider::new(secret_id, secret_key, region)),
        "s3" => Box::new(S3Provider::new(secret_id, secret_key, region)),
        _ => panic!("Unsupported provider: {}", provider),
    }
}
