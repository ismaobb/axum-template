use core::fmt;

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use thiserror::Error;

use super::ApiOk;

#[derive(Debug, Error)]
pub enum ApiErr {
    MissingToken,
    #[error(transparent)]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    Forbidden,
    WrongCredentials,
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    HttpRequestError(#[from] reqwest::Error),
}

impl IntoResponse for ApiErr {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(?self, "API error");

        let (status_code, message) = match self {
            ApiErr::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token".to_string()),
            ApiErr::InvalidToken(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            ApiErr::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".to_string()),
            ApiErr::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            ApiErr::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
            ApiErr::DbError(db_err) => (StatusCode::INTERNAL_SERVER_ERROR, db_err.to_string()),
            ApiErr::HttpRequestError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
            }
        };

        (
            status_code,
            Json(ApiOk::<()> {
                code: -1,
                data: None,
                message: Some(message),
            }),
        )
            .into_response()
    }
}

impl fmt::Display for ApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
