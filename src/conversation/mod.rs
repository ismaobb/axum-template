use axum::{routing::{get, post}, Router};

mod service;
mod dto;

pub fn controller() -> Router {
    Router::new().route("/conversation", get(service::conversation_handler))
}
