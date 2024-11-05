use std::sync::Arc;

use axum::{
    extract::{Request, State},
    RequestExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{
    core::{jwt::jwt_decode, ApiErr, AppState},
    extract::{Passthrough, User},
};

/// Bearer认证成功，写入到请求扩展，方便路由鉴权
/// ```
/// req.extensions_mut().insert(crate::user::User)
/// ```
pub async fn auth_token(
    State(state): State<Arc<AppState>>,
    passthrough: Passthrough,
    mut req: Request,
) -> Result<Request, ApiErr> {
    if passthrough.0 {
        return Ok(req);
    }
    tracing::debug!(?state);
    let TypedHeader(Authorization(bearer)) = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| ApiErr::MissingToken)?;

    tracing::info!(?bearer);

    jwt_decode(bearer.token()).map(|c| {
        req.extensions_mut().insert(User {
            id: c.id,
            username: c.username,
            role: c.role,
            device: c.device,
        });
        req
    })
}
