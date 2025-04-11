use crate::apis::v1::chat::chat_service::handle_chat;

use axum::{routing::post, Router};

pub fn routes_chat() -> Router {
    Router::new().route("/chat", post(handle_chat))
}
