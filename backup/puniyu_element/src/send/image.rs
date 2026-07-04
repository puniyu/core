use crate::{Element, ElementType};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: Bytes,
	/// 图片文件名
	pub file_name: String,
	/// 图片外显
	pub summary: String,
}

impl ImageElement {
	pub fn new(
		file: impl Into<Bytes>,
		file_name: impl Into<String>,
		summary: Option<String>,
	) -> Self {
		let file_name = file_name.into();
		let summary = summary.unwrap_or_else(|| file_name.clone());
		Self { file: file.into(), file_name, summary }
	}
}

impl Element for ImageElement {
	fn r#type(&self) -> ElementType {
		ElementType::Image
	}
}
