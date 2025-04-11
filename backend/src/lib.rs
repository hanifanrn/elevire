use axum::{
    http::{header, HeaderValue, Method},
    Router,
};
use tower_http::cors::CorsLayer;

pub mod apis;
pub mod utils;

pub fn app() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:8080"), // for development only
            HeaderValue::from_static("http://localhost:8082"), // for development only
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    Router::new()
        .merge(apis::v1::chat::chat_controller::routes_chat())
        .fallback(utils::fallback::fallback)
        .layer(cors_layer)
}
