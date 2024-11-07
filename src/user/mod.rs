use std::sync::Arc;

use axum::{
    extract::{Query, Request},
    routing::get,
    Extension, Router,
};
use serde::Deserialize;

use crate::{
    core::{ApiErr, ApiOk, AppState, Role, RoleState},
    extract::Claims,
    middleware::{auth_role, role_check},
};

mod view;

pub fn controller() -> Router {
    let role_state = Arc::new(RoleState(Role(1)));
    Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/", get(find_users).post(create_user))
                .route("/:id", get(find_users)),
        )
        .route_layer(axum::middleware::map_request_with_state(
            role_state, auth_role,
        ))
        .layer(axum::middleware::from_fn(|state, req, next| {
            role_check(state, req, next, Role(1))
        }))
}

#[derive(Deserialize, Debug)]
struct UserQuery {
    line: u8,
    roll_id: u32,
}

async fn create_user() -> Result<ApiOk<Claims>, ApiErr> {
    Err(ApiErr::WrongCredentials)
}

async fn find_users(
    Query(query): Query<UserQuery>,
    Extension(state): Extension<Arc<AppState>>,
    claims: Claims,
    _req: Request,
) -> Result<ApiOk<Vec<Claims>>, ApiErr> {
    tracing::debug!(?query);
    tracing::info!(?claims);
    tracing::info!(name = "find_users", "{}", state.conn);

    let api_response = ApiOk::from(vec![claims]);

    Ok(api_response)
}
