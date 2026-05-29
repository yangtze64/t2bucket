mod crypto;
mod store;
mod providers;

use providers::create_provider;
use store::Connection;

#[tauri::command]
fn list_connections() -> Vec<Connection> {
    store::list_connections()
}

#[tauri::command]
fn add_connection(
    name: String,
    secret_id: String,
    secret_key: String,
    region: String,
    provider: String,
) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("连接名称不能为空".to_string());
    }
    if secret_id.trim().is_empty() {
        return Err("密钥ID不能为空".to_string());
    }
    if secret_key.trim().is_empty() {
        return Err("密钥Secret不能为空".to_string());
    }
    store::add_connection(&name, &secret_id, &secret_key, &region, &provider)
}

#[tauri::command]
fn update_connection(
    id: String,
    name: String,
    secret_id: String,
    secret_key: String,
    region: String,
    provider: String,
) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("连接名称不能为空".to_string());
    }
    if secret_id.trim().is_empty() {
        return Err("密钥ID不能为空".to_string());
    }
    if secret_key.trim().is_empty() {
        return Err("密钥Secret不能为空".to_string());
    }
    store::update_connection(&id, &name, &secret_id, &secret_key, &region, &provider)
}

#[tauri::command]
fn delete_connection(id: String) -> Result<(), String> {
    store::delete_connection(&id)
}

#[tauri::command]
fn test_connection(
    secret_id: String,
    secret_key: String,
    region: String,
    provider: String,
) -> Result<Vec<String>, String> {
    let storage = create_provider(&provider, &secret_id, &secret_key, &region);
    storage.list_buckets()
}

#[tauri::command]
fn list_buckets(connection_id: String) -> Result<Vec<String>, String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在".to_string())?;
    let storage = create_provider(&conn.provider, &conn.secret_id, &conn.secret_key, &conn.region);
    storage.list_buckets()
}

#[tauri::command]
fn list_objects(
    connection_id: String,
    bucket: String,
    prefix: Option<String>,
    delimiter: Option<String>,
) -> Result<serde_json::Value, String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在".to_string())?;
    let storage = create_provider(&conn.provider, &conn.secret_id, &conn.secret_key, &conn.region);
    let (prefixes, objects) = storage.list_objects(
        &bucket,
        &prefix.unwrap_or_default(),
        &delimiter.unwrap_or_else(|| "/".to_string()),
    )?;
    Ok(serde_json::json!({
        "prefixes": prefixes,
        "items": objects.iter().map(|o| serde_json::json!({
            "key": o.key,
            "size": o.size,
            "lastModified": o.last_modified,
            "isDir": o.is_dir,
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
fn get_object(connection_id: String, bucket: String, key: String) -> Result<Vec<u8>, String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在".to_string())?;
    let storage = create_provider(&conn.provider, &conn.secret_id, &conn.secret_key, &conn.region);
    storage.get_object(&bucket, &key)
}

#[tauri::command]
fn put_object(
    connection_id: String,
    bucket: String,
    key: String,
    content: Vec<u8>,
) -> Result<(), String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在".to_string())?;
    let storage = create_provider(&conn.provider, &conn.secret_id, &conn.secret_key, &conn.region);
    storage.put_object(&bucket, &key, &content)
}

#[tauri::command]
fn delete_object(
    connection_id: String,
    bucket: String,
    key: String,
) -> Result<(), String> {
    let conn = store::get_connection(&connection_id).ok_or("连接不存在".to_string())?;
    let storage = create_provider(&conn.provider, &conn.secret_id, &conn.secret_key, &conn.region);
    storage.delete_object(&bucket, &key)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_connections,
            add_connection,
            update_connection,
            delete_connection,
            test_connection,
            list_buckets,
            list_objects,
            get_object,
            put_object,
            delete_object,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
