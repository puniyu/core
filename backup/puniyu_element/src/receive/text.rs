use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement<'t> {
	/// 文本元素内容
	pub text: &'t str,
}

impl<'t> Element for TextElement<'t> {
	fn r#type(&self) -> ElementType {
		ElementType::Text
	}
}
impl<'t> From<TextElement<'t>> for String {
	fn from(elem: TextElement<'t>) -> Self {
		elem.text.to_string()
	}
}
impl<'t> From<TextElement<'t>> for &'t str {
	fn from(elem: TextElement<'t>) -> Self {
		elem.text
	}
}

impl<'t> From<&'t str> for TextElement<'t> {
	fn from(text: &'t str) -> Self {
		Self { text }
	}
}
