use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use serde::Serialize;

use crate::core::{ApiErr, Role};

#[derive(Debug, Clone, Serialize, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub device: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = ApiErr;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<User>()
            .cloned()
            .map_or(Ok(Self::default()), Ok)
    }
}
