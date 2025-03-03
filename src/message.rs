use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::{common::types::ServiceTransport, GatewayState};

mod dtos;
pub mod message_controller;

pub fn message_routes<T: ServiceTransport>() -> Router<GatewayState<T>> {
    let router = Router::new()
        .route("/messages/{user_id}", get(message_controller::get_messages))
        .route("/message/send", post(message_controller::send_message))
        .route(
            "/message/{message_id}",
            patch(message_controller::edit_message).delete(message_controller::delete_message),
        );

    router
}
