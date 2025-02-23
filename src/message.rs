use axum::{
    routing::{get, patch, post},
    Router,
};
use message_controller::MessageController;

mod message_controller;

pub fn setup_message(app: Router) {
    let message_controller = MessageController::new();

    app.route("messages/{user_id}", get(message_controller.get_messages))
        .route("message/send", post(message_controller.send_message))
        .route("message/edit", patch(message_controller.edit_message));
}
