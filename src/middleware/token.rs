use std::sync::Arc;

use axum::{extract::Request, Extension, RequestExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::{
    core::{ApiErr, AppState},
    extract::{Claims, Passthrough},
};

/// Bearer认证成功，写入到请求扩展，方便路由鉴权
/// ```
/// req.extensions_mut().insert(crate::user::User)
/// ```
pub async fn auth_token(
    Extension(app_state): Extension<Arc<AppState>>,
    passthrough: Passthrough,
    mut req: Request,
) -> Result<Request, ApiErr> {
    if passthrough.0 {
        return Ok(req);
    }
    let TypedHeader(Authorization(bearer)) = req
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| ApiErr::MissingToken)?;

    tracing::debug!(?bearer);

    Claims::decode(bearer.token(), &app_state.config.jwt_secret).map(|claims| {
        req.extensions_mut().insert(claims);
        req
    })
}
