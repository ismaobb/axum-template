use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiOk<T: Serialize> {
	#[serde(rename = "result")]
	pub code: i32,
	pub data: Option<T>,
	pub message: Option<String>,
}

impl<T: Serialize> From<T> for ApiOk<T> {
	fn from(value: T) -> Self {
		Self {
			code: Default::default(),
			data: Some(value),
			message: None,
		}
	}
}

impl<T: Serialize> IntoResponse for ApiOk<T> {
	fn into_response(self) -> axum::response::Response {
		Json(self).into_response()
	}
}
