use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserQueryDto {
	pub id: Option<i32>,
	pub username: Option<String>,
}
