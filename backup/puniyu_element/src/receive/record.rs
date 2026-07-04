use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement<'r> {
	/// 语音元素
	pub file: Bytes,
	/// 文件名
	pub file_name: &'r str
}

impl<'r> Element for RecordElement<'r> {
	fn r#type(&self) -> ElementType {
		ElementType::Record
	}
}
