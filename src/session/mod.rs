use axum::{routing::get, Router};

mod dto;
mod service;

pub fn controller() -> Router {
	Router::new().nest("/sessions", Router::new().route("/", get(service::get_sessions)))
}
