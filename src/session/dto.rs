use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SessionQueryDto {
    pub user_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>, // iso 8601
}
