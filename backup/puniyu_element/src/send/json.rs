use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: String,
}

impl JsonElement {
	pub fn new(data: impl Into<String>) -> Self {
		Self { data: data.into() }
	}
}

impl From<&str> for JsonElement {
	fn from(data: &str) -> Self {
		Self::new(data)
	}
}

impl From<String> for JsonElement {
	fn from(data: String) -> Self {
		Self::new(data)
	}
}

impl From<JsonElement> for String {
	fn from(element: JsonElement) -> Self {
		element.data
	}
}

impl AsRef<str> for JsonElement {
	fn as_ref(&self) -> &str {
		&self.data
	}
}

impl Element for JsonElement {
	fn r#type(&self) -> ElementType {
		ElementType::Json
	}
}
