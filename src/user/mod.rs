use std::sync::Arc;

use crate::{
	core::{Role, RoleState},
	middleware::role_check,
};
use axum::{routing::get, Router};

pub mod dto;
mod service;

pub fn controller() -> Router {
	let role_state = Arc::new(RoleState(Role(1)));
	Router::new()
		.nest(
			"/users",
			Router::new()
				.route("/", get(service::find_users))
				.route("/:id", get(service::find_user)),
		)
		// .route_layer(axum::middleware::map_request_with_state(
		//     role_state, auth_role,
		// ))
		.layer(axum::middleware::from_fn(|state, req, next| {
			role_check(state, req, next, Role(1))
		}))
}
