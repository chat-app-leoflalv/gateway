use axum::Json;
use serde_json::{json, Value};

pub async fn get_messages() -> Json<Value> {
    Json(json!({ "data": "get_messages" }))
}

pub async fn send_message() -> Json<Value> {
    Json(json!({ "data": "send_message" }))
}

pub async fn delete_message() -> Json<Value> {
    Json(json!({ "data": "delete_message" }))
}

pub async fn edit_message() -> Json<Value> {
    Json(json!({ "data": "edit_message" }))
}
