use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub secret_id: String,
    pub secret_key: String,
    pub region: String,
    pub provider: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredConnection {
    id: String,
    name: String,
    enc_secret_id: String,
    enc_secret_key: String,
    region: String,
    provider: String,
    created_at: i64,
}

fn store_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    let dir = PathBuf::from(&home).join(".t2bucket");
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("STORE_WARN Failed to create dir: {}", e);
    }
    dir.join("connections.json")
}

fn set_permissions(path: &PathBuf) {
    #[cfg(unix)]
    {
        if let Err(e) = fs::set_permissions(path, fs::Permissions::from_mode(0o600)) {
            eprintln!("STORE_WARN Failed to set permissions: {}", e);
        }
    }
}

fn load_stored() -> Vec<StoredConnection> {
    let path = store_path();
    if !path.exists() {
        eprintln!("STORE_DEBUG No connections.json found");
        return vec![];
    }
    let data = fs::read_to_string(&path).unwrap_or_default();
    let result: Vec<StoredConnection> = serde_json::from_str(&data).unwrap_or_default();
    eprintln!("STORE_DEBUG Parsed {} stored connections", result.len());
    result
}

fn save_stored(conns: &[StoredConnection]) {
    let path = store_path();
    let data = serde_json::to_string_pretty(conns).unwrap_or_default();
    if let Err(e) = fs::write(&path, data) {
        eprintln!("STORE_ERROR Failed to save connections: {}", e);
    } else {
        set_permissions(&path);
    }
}

fn to_stored(conn: &Connection) -> StoredConnection {
    let enc_secret_id = crate::crypto::encrypt(&conn.secret_id);
    let enc_secret_key = crate::crypto::encrypt(&conn.secret_key);
    eprintln!("STORE_DEBUG Encrypting for connection: {}", conn.name);
    StoredConnection {
        id: conn.id.clone(),
        name: conn.name.clone(),
        enc_secret_id,
        enc_secret_key,
        region: conn.region.clone(),
        provider: conn.provider.clone(),
        created_at: conn.created_at,
    }
}

fn from_stored(s: &StoredConnection) -> Option<Connection> {
    let secret_id = match crate::crypto::decrypt(&s.enc_secret_id) {
        Ok(secret_id) => secret_id,
        Err(e) => {
            eprintln!("STORE_ERROR Failed to decrypt secret_id for {}: {}", s.name, e);
            return None;
        }
    };
    let secret_key = match crate::crypto::decrypt(&s.enc_secret_key) {
        Ok(secret_key) => secret_key,
        Err(e) => {
            eprintln!("STORE_ERROR Failed to decrypt secret_key for {}: {}", s.name, e);
            return None;
        }
    };
    eprintln!("STORE_DEBUG Decrypted connection: {}", s.name);
    Some(Connection {
        id: s.id.clone(),
        name: s.name.clone(),
        secret_id,
        secret_key,
        region: s.region.clone(),
        provider: s.provider.clone(),
        created_at: s.created_at,
    })
}

pub fn list_connections() -> Vec<Connection> {
    eprintln!("STORE_DEBUG list_connections() called");
    load_stored().iter().filter_map(from_stored).collect()
}

pub fn add_connection(
    name: &str,
    secret_id: &str,
    secret_key: &str,
    region: &str,
    provider: &str,
) -> Result<String, String> {
    eprintln!("STORE_DEBUG add_connection called: name={}", name);
    let id = uuid::Uuid::new_v4().to_string();
    let conn = Connection {
        id: id.clone(),
        name: name.to_string(),
        secret_id: secret_id.to_string(),
        secret_key: secret_key.to_string(),
        region: region.to_string(),
        provider: provider.to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };
    let mut stored = load_stored();
    stored.push(to_stored(&conn));
    save_stored(&stored);
    Ok(id)
}

pub fn update_connection(
    id: &str,
    name: &str,
    secret_id: &str,
    secret_key: &str,
    region: &str,
    provider: &str,
) -> Result<(), String> {
    eprintln!("STORE_DEBUG update_connection called: id={}", id);
    let mut stored = load_stored();
    let item = stored.iter_mut().find(|c| c.id == id).ok_or("连接不存在")?;
    item.name = name.to_string();
    item.enc_secret_id = crate::crypto::encrypt(secret_id);
    item.enc_secret_key = crate::crypto::encrypt(secret_key);
    item.region = region.to_string();
    item.provider = provider.to_string();
    save_stored(&stored);
    Ok(())
}

pub fn delete_connection(id: &str) -> Result<(), String> {
    let mut stored = load_stored();
    let len = stored.len();
    stored.retain(|c| c.id != id);
    if stored.len() == len {
        return Err("连接不存在".to_string());
    }
    save_stored(&stored);
    Ok(())
}

pub fn get_connection(id: &str) -> Option<Connection> {
    eprintln!("STORE_DEBUG get_connection called: id={}", id);
    load_stored().iter().find(|c| c.id == id).and_then(from_stored)
}
