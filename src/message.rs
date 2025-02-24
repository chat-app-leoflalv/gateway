use axum::{
    routing::{get, patch, post},
    Router,
};

mod message_controller;

pub trait MessageRouterExt {
    fn setup_message(self) -> Self;
}

impl MessageRouterExt for Router {
    fn setup_message(self) -> Self {
        self.route("/messages/{user_id}", get(message_controller::get_messages))
            .route(
                "/message/{message_id}",
                patch(message_controller::edit_message).delete(message_controller::delete_message),
            )
            .route("/message/send", post(message_controller::send_message))
    }
}
