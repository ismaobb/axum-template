use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::extract::User;

use super::{ApiErr, Role};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i32,
    pub device: Option<String>,
    pub username: String,
    pub role: Role,
    iat: i64,
    exp: i64,
    sub: String,
}

pub fn jwt_encode(user: &User, secret: &str, hours: i64) -> Result<String, ApiErr> {
    let now = Utc::now();

    let claims = &Claims {
        iat: now.timestamp(),
        exp: (now + Duration::hours(hours)).timestamp(),
        id: user.id,
        sub: "axum-template".to_owned(),
        role: user.role.clone(),
        device: user.device.to_owned(),
        username: user.username.clone(),
    };

    tracing::info!(?claims);
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(ApiErr::InvalidToken)?;

    tracing::info!(?token);
    Ok(token)
}

pub fn jwt_decode(token: &str, secret: &str) -> Result<Claims, ApiErr> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(ApiErr::InvalidToken)
    .map(|t| {
        tracing::info!(?t);
        t.claims
    })
}
