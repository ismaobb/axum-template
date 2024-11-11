use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entity::{context, sessions};

#[derive(Debug, Deserialize)]
pub struct SessionQueryDto {
    pub user_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>, // iso 8601
}

#[derive(Debug, Serialize)]
pub struct SessionResponseDto {
    pub session: sessions::Model,
    pub contexts: Vec<context::Model>,
}
