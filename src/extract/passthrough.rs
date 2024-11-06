use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, MatchedPath},
    http::{request::Parts, Method},
    Extension, RequestPartsExt,
};

use crate::core::AppState;

/// # 路由白名单规则：
/// ### 只校验Method::GET
/// ### 通过MatchedPath精准匹配
pub struct Passthrough(pub bool);

#[async_trait]
impl<S> FromRequestParts<S> for Passthrough
where
    S: Send + Sync,
{
    type Rejection = ();
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从parts中提取AppState
        let app_state = parts.extract::<Extension<Arc<AppState>>>().await.unwrap();
        // 如果请求方法不是GET，则返回Passthrough(false)
        if !parts.method.eq(Method::GET.as_str()) {
            return Ok(Passthrough(false));
        }

        if let Some(path) = parts.extensions.get::<MatchedPath>() {
            if let Some(whitelist) = app_state.config.whitelist.clone() {
                let passthrough = whitelist.iter().any(|pattern| pattern.eq(path.as_str()));
                tracing::debug!(?passthrough);
                return Ok(Passthrough(passthrough));
            }
            Ok(Passthrough(false))
        } else {
            Ok(Passthrough(false))
        }
    }
}
