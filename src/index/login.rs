use std::{borrow::Cow, sync::Arc};

use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::{
    core::{ApiErr, ApiOk, AppState, Role},
    extract::Claims,
};

#[derive(Debug, Deserialize)]
pub struct LoginBody<'a> {
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub device: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    WithRejection(Json(login_body), _): WithRejection<Json<LoginBody<'_>>, ApiErr>,
) -> Result<ApiOk<LoginResponse>, ApiErr> {
    let LoginBody {
        username,
        password,
        device,
    } = login_body;

    tracing::info!(?device);
    if username.eq("schemer") && password.eq("14e1b600b1fd579f47433b88e8d85291") {
        let access_token = Claims::new(
            1,
            device,
            username.into_owned(),
            Role(1),
            app_state.config.jwt_expire_in_hours,
        )
        .encode(&app_state.config.jwt_secret)?;
        Ok(ApiOk::from(LoginResponse { access_token }))
    } else {
        Err(ApiErr::WrongCredentials)
    }
}
