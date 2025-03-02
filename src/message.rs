use axum::{routing::get, Router};

use crate::{common::types::ServiceTransport, GatewayState};

mod dtos;
pub mod message_controller;

pub fn message_routes<T: ServiceTransport>() -> Router<GatewayState<T>> {
    let router = Router::new().route("/messages/{user_id}", get(message_controller::get_messages));

    router
}
