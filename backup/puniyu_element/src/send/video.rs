use bytes::Bytes;
use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement {
	/// 视频元素
	pub file: Bytes,
	/// 视频文件名
	pub file_name: String,
}

impl VideoElement {
	pub fn new(file: impl Into<Bytes>, file_name: impl Into<String>) -> Self {
		Self { file: file.into(), file_name: file_name.into() }
	}
}

impl Element for VideoElement {
	fn r#type(&self) -> ElementType {
		ElementType::Video
	}
}
