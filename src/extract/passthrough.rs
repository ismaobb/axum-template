use axum::{
    async_trait,
    extract::{FromRequestParts, MatchedPath},
    http::{request::Parts, Method},
};

/// # 路由白名单规则：
/// ### 只校验Method::GET
/// ### 通过MatchedPath精准匹配
const WHITELIST_PATHS: &[&str] = &["/users/:id", "/public", "/healthcheck"];

pub struct Passthrough(pub bool);

#[async_trait]
impl<S> FromRequestParts<S> for Passthrough
where
    S: Send + Sync,
{
    type Rejection = ();
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if !parts.method.eq(Method::GET.as_str()) {
            return Ok(Passthrough(false));
        }
        if let Some(path) = parts.extensions.get::<MatchedPath>() {
            let passthrough = WHITELIST_PATHS
                .iter()
                .any(|&pattern| pattern.eq(path.as_str()));
            tracing::debug!(?passthrough);
            Ok(Passthrough(passthrough))
        } else {
            Ok(Passthrough(false))
        }
    }
}
