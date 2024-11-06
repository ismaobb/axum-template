use std::{borrow::Cow, sync::Arc};

use axum::{Extension, Json};
use axum_extra::extract::WithRejection;
use serde::{Deserialize, Serialize};

use crate::{
    core::{jwt::jwt_encode, ApiErr, ApiOk, AppState, Role},
    extract::User,
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
        let access_token = jwt_encode(
            &User {
                id: 1,
                username: username.into_owned(),
                role: Role(1),
                device,
            },
            &app_state.config.jwt_secret,
            app_state.config.jwt_expire_in_hours,
        )?;
        Ok(ApiOk::from(LoginResponse { access_token }))
    } else {
        Err(ApiErr::WrongCredentials)
    }
}
