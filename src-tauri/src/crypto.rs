use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hex;
use rand::RngCore;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const NONCE_SIZE: usize = 12;

fn key_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(&home).join(".t2bucket");
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("CRYPTO_WARN Failed to create dir: {}", e);
    }
    dir.join(".key")
}

fn set_permissions(path: &PathBuf) {
    #[cfg(unix)]
    {
        if let Err(e) = fs::set_permissions(path, fs::Permissions::from_mode(0o600)) {
            eprintln!("CRYPTO_WARN Failed to set permissions: {}", e);
        }
    }
}

fn load_or_create_key() -> Vec<u8> {
    let path = key_path();
    eprintln!("CRYPTO_DEBUG Loading key from {:?}", path);
    if path.exists() {
        let text = fs::read_to_string(&path).unwrap_or_default();
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            if let Ok(bytes) = hex::decode(trimmed) {
                if bytes.len() == 32 {
                    eprintln!("CRYPTO_DEBUG Successfully decoded key");
                    return bytes;
                }
            }
        }
    }
    eprintln!("CRYPTO_DEBUG Creating new key");
    let mut key = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key);
    let hex_str = hex::encode(&key);
    if let Err(e) = fs::write(&path, &hex_str) {
        eprintln!("CRYPTO_ERROR Failed to write key file: {}", e);
    } else {
        set_permissions(&path);
    }
    key
}

pub fn encrypt(plaintext: &str) -> String {
    if plaintext.is_empty() {
        return String::new();
    }
    let key_bytes = load_or_create_key();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failure");

    let mut combined = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    let result = BASE64.encode(&combined);
    eprintln!("CRYPTO_DEBUG Encrypted {} chars -> {} chars", plaintext.len(), result.len());
    result
}

pub fn decrypt(encoded: &str) -> Result<String, String> {
    if encoded.is_empty() {
        return Ok(String::new());
    }
    eprintln!("CRYPTO_DEBUG Decrypting {} chars", encoded.len());
    let key_bytes = load_or_create_key();
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let combined = match BASE64.decode(encoded) {
        Ok(c) => c,
        Err(_e) => {
            eprintln!("CRYPTO_ERROR Base64 decode failed");
            return Err(format!("解密失败: Base64 解码失败"));
        }
    };

    if combined.len() < NONCE_SIZE {
        eprintln!("CRYPTO_ERROR Combined too short: {} < {}", combined.len(), NONCE_SIZE);
        return Err(format!("解密失败: 数据格式错误"));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = match cipher.decrypt(nonce, ciphertext) {
        Ok(b) => b,
        Err(_e) => {
            eprintln!("CRYPTO_ERROR AES-GCM decrypt failed");
            return Err(format!("解密失败: AES-GCM 验证失败，请重新输入凭证"));
        }
    };

    let result = String::from_utf8_lossy(&decrypted_bytes).to_string();
    eprintln!("CRYPTO_DEBUG Decrypted to {} chars", result.len());
    Ok(result)
}
