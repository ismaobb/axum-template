use std::sync::Arc;

use axum::{
    body::Body, extract::State, http::Request, middleware::Next, response::Response, Extension,
};

use crate::{
    core::{ApiErr, AppState, Role, RoleState},
    extract::{Claims, Passthrough},
};

pub async fn auth_role(
    State(role_state): State<Arc<RoleState>>,
    Extension(app_state): Extension<Arc<AppState>>,
    claims: Claims,
    passthrough: Passthrough,
    req: Request<Body>,
) -> Result<Request<Body>, ApiErr> {
    if passthrough.0 {
        return Ok(req);
    }

    tracing::debug!(?app_state);
    tracing::debug!(?role_state);

    if claims.role == role_state.0 {
        Ok(req)
    } else {
        Err(ApiErr::Forbidden)
    }
}

pub async fn role_check(
    Extension(_app_state): Extension<Arc<AppState>>,
    req: axum::extract::Request,
    next: Next,
    required_roles: Role,
) -> Result<Response, ApiErr> {
    tracing::debug!(?required_roles);
    Ok(next.run(req).await)
}
