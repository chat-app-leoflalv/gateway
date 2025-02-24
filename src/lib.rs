use axum::Router;
use message::MessageRouterExt;

mod message;

pub fn setup() -> Router {
    let app = Router::new().setup_message();

    app
}
