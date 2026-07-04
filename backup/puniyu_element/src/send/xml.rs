use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement {
	/// Xml数据，未序列化
	pub data: String,
}

impl XmlElement {
	pub fn new(data: impl Into<String>) -> Self {
		Self { data: data.into() }
	}
}

impl From<&str> for XmlElement {
	fn from(data: &str) -> Self {
		Self::new(data)
	}
}

impl From<String> for XmlElement {
	fn from(data: String) -> Self {
		Self::new(data)
	}
}

impl From<XmlElement> for String {
	fn from(element: XmlElement) -> Self {
		element.data
	}
}

impl AsRef<str> for XmlElement {
	fn as_ref(&self) -> &str {
		&self.data
	}
}

impl Element for XmlElement {
	fn r#type(&self) -> ElementType {
		ElementType::Xml
	}
}
