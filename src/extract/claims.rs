use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::{ApiErr, Role};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Claims {
	pub id: i32,
	pub username: String,
	pub role: Role,
	pub device: Option<String>,
	iat: u64,
	exp: u64,
	sub: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
	type Rejection = ();

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
		parts.extensions.get::<Self>().cloned().map_or(Ok(Self::default()), Ok)
	}
}

impl Claims {
	pub fn new(id: i32, device: Option<String>, username: String, role: Role, hours: i64) -> Self {
		let iat = jsonwebtoken::get_current_timestamp();
		Self {
			id,
			device,
			username,
			role,
			iat,
			exp: iat + Duration::hours(hours).num_seconds() as u64,
			sub: "axum-template".to_owned(),
		}
	}

	pub fn encode(&self, secret: &str) -> Result<String, ApiErr> {
		tracing::info!(?self);
		let token =
			encode(&Header::default(), self, &EncodingKey::from_secret(secret.as_bytes())).map_err(ApiErr::InvalidToken)?;

		tracing::info!(?token);
		Ok(token)
	}

	pub fn decode(token: &str, secret: &str) -> Result<Self, ApiErr> {
		decode::<Self>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
			.map_err(ApiErr::InvalidToken)
			.map(|token_data| {
				tracing::info!(?token_data.claims);
				token_data.claims
			})
	}
}
