use bytes::Bytes;
use crate::{Element, ElementType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement {
	/// 语音元素
	pub file: Bytes,
	/// 语音文件名
	pub file_name: String,
}

impl RecordElement {
	pub fn new(file: impl Into<Bytes>, file_name: impl Into<String>) -> Self {
		Self { file: file.into(), file_name: file_name.into() }
	}
}

impl Element for RecordElement {
	fn r#type(&self) -> ElementType {
		ElementType::Record
	}
}
