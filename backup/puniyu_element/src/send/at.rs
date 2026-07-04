use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtElement {
	/// 目标id
	pub target_id: String,
}

impl AtElement {
	pub fn new(target_id: impl Into<String>) -> Self {
		Self { target_id: target_id.into() }
	}

	/// 是否为艾特全体。
	pub fn is_everyone(&self) -> bool {
		matches!(self.target_id.as_str(), "all")
	}
}

impl From<&str> for AtElement {
	fn from(target_id: &str) -> Self {
		Self::new(target_id)
	}
}

impl From<String> for AtElement {
	fn from(target_id: String) -> Self {
		Self::new(target_id)
	}
}

impl From<AtElement> for String {
	fn from(element: AtElement) -> Self {
		element.target_id
	}
}

impl AsRef<str> for AtElement {
	fn as_ref(&self) -> &str {
		&self.target_id
	}
}

impl AtElement {
	/// 构造艾特全体元素。
	pub fn everyone() -> Self {
		Self::new("all")
	}
}

impl Element for AtElement {
	fn r#type(&self) -> ElementType {
		ElementType::At
	}
}
