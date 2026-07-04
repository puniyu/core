use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
	/// 文本元素内容
	pub text: String,
}

impl TextElement {
	pub fn new(text: impl Into<String>) -> Self {
		Self { text: text.into() }
	}
}

impl From<&str> for TextElement {
	fn from(text: &str) -> Self {
		Self::new(text)
	}
}

impl From<String> for TextElement {
	fn from(text: String) -> Self {
		Self::new(text)
	}
}

impl From<TextElement> for String {
	fn from(element: TextElement) -> Self {
		element.text
	}
}

impl AsRef<str> for TextElement {
	fn as_ref(&self) -> &str {
		&self.text
	}
}

impl Element for TextElement {
	fn r#type(&self) -> ElementType {
		ElementType::Text
	}
}
