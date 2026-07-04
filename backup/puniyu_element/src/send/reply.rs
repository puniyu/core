use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
	/// 回复元素id
	pub message_id: String,
}

impl ReplyElement {
	pub fn new(message_id: impl Into<String>) -> Self {
		Self { message_id: message_id.into() }
	}
}

impl From<&str> for ReplyElement {
	fn from(message_id: &str) -> Self {
		Self::new(message_id)
	}
}

impl From<String> for ReplyElement {
	fn from(message_id: String) -> Self {
		Self::new(message_id)
	}
}

impl From<ReplyElement> for String {
	fn from(element: ReplyElement) -> Self {
		element.message_id
	}
}

impl AsRef<str> for ReplyElement {
	fn as_ref(&self) -> &str {
		&self.message_id
	}
}

impl Element for ReplyElement {
	fn r#type(&self) -> ElementType {
		ElementType::Reply
	}
}
