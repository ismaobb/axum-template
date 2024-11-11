use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    core::{ApiErr, ApiOk, AppState},
    entity::{
        prelude::*,
        users::{self, Model},
    },
};

use super::dto::UserQueryDto;

pub async fn find_users(
    Query(query): Query<UserQueryDto>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<ApiOk<Vec<Model>>, ApiErr> {
    tracing::debug!(?query);
    let users = Users::find()
        .filter(
            Condition::all()
                .add_option(
                    query
                        .username
                        .map(|username| users::Column::Username.eq(username)),
                )
                .add_option(query.id.map(|id| users::Column::Id.eq(id))),
        )
        .all(&state.conn)
        .await?;

    Ok(ApiOk::from(users))
}

pub async fn find_user(
    Path(id): Path<i32>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<ApiOk<Option<Model>>, ApiErr> {
    let user = Users::find_by_id(id).one(&state.conn).await?;

    Ok(ApiOk::from(user))
}
