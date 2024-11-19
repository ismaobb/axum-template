use chrono::NaiveDateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	if let Some(date) = date {
		let formatted_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
		serializer.serialize_str(&formatted_date)
	} else {
		serializer.serialize_none()
	}
}

// 反序列化（如果需要）
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<&str> = Option::deserialize(deserializer)?;
	if let Some(s) = s {
		NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
			.map(Some)
			.map_err(D::Error::custom)
	} else {
		Ok(None)
	}
}
