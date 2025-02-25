use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use super::dtos::{
    edit_message_request_dto::EditMessageRequestDto,
    send_message_request_dto::SendMessageRequestDto,
};

pub async fn get_messages(Path(user_id): Path<Uuid>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({ "data": user_id.to_string() })))
}

pub async fn send_message(
    Json(body): Json<SendMessageRequestDto>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(json!({ "body": body })))
}

pub async fn delete_message(Path(message_id): Path<Uuid>) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(
        json!({ "data": "delete_message", "message_id": message_id.to_string() }),
    ))
}

pub async fn edit_message(
    Path(message_id): Path<Uuid>,
    Json(body): Json<EditMessageRequestDto>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(
        json!({ "message_id": message_id.to_string(), "body": body }),
    ))
}
