use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ConversationRequest {
	pub prompt: String,
}
