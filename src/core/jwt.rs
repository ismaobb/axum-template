use chrono::Utc;
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

struct Key {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Key {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn jwt_encode(user: &User) -> Result<String, ApiErr> {
    let now = Utc::now();
    let claims = &Claims {
        iat: now.timestamp(),
        // exp: (now + Duration::days(1)).timestamp(),
        exp: 2000000000,
        id: user.id,
        sub: "axum-template".to_owned(),
        role: user.role.clone(),
        device: user.device.to_owned(),
        username: user.username.clone(),
    };

    tracing::info!(?claims);
    let key = Key::new(b"secret");
    let token = encode(&Header::default(), claims, &key.encoding).map_err(ApiErr::InvalidToken)?;

    tracing::info!(?token);
    Ok(token)
}

pub fn jwt_decode(token: &str) -> Result<Claims, ApiErr> {
    let key = Key::new(b"secret");
    decode::<Claims>(token, &key.decoding, &Validation::default())
        .map_err(ApiErr::InvalidToken)
        .map(|t| {
            tracing::info!(?t);
            t.claims
        })
}
