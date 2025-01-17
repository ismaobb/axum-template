use axum::{
	routing::get,
	Router,
};

mod dto;
mod service;

pub fn controller() -> Router {
	Router::new().route("/conversation", get(service::conversation_handler))
}
