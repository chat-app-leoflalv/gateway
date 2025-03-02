use axum::Router;
use common::{state::GatewayState, types::ServiceTransport};
use message::message_routes;

pub mod common;
pub mod config;
pub mod core;
mod message;

pub fn routes<T: ServiceTransport>() -> Router<GatewayState<T>> {
    let app = Router::new().merge(message_routes());

    app
}
