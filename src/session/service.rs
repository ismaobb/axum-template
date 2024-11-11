use std::sync::Arc;

use axum::{extract::Query, Extension};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::{
    core::{ApiErr, ApiOk, AppState},
    entity::{prelude::*, sessions},
};

use super::dto::{SessionQueryDto, SessionResponseDto};

pub async fn get_sessions(
    Query(query): Query<SessionQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<ApiOk<Vec<SessionResponseDto>>, ApiErr> {
    let sessions = Sessions::find()
        .find_with_related(Context)
        .filter(
            Condition::all()
                .add_option(
                    query
                        .created_at
                        .map(|created_at| sessions::Column::CreatedAt.gte(created_at)),
                )
                .add_option(
                    query
                        .user_id
                        .map(|user_id| sessions::Column::UserId.eq(user_id)),
                ),
        )
        .all(&app_state.conn)
        .await?
        .into_iter()
        .map(|(session, contexts)| SessionResponseDto { session, contexts })
        .collect::<Vec<SessionResponseDto>>();

    Ok(ApiOk::from(sessions))
}
