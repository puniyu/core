//! 接收消息元素模块
//!
//! 用于表示接收消息元素。

use crate::{Element, ElementType, codegen_reexport};
use serde::{Deserialize, Serialize};

codegen_reexport! {
	text => TextElement,
	at => AtElement,
	reply => ReplyElement,
	face => FaceElement,
	image => ImageElement,
	file => FileElement,
	video => VideoElement,
	record => RecordElement,
	json => JsonElement,
	xml => XmlElement
}

/// 接收元素枚举。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'e"))]
pub enum Elements<'e> {
	Text(TextElement<'e>),
	At(AtElement<'e>),
	Reply(ReplyElement<'e>),
	Face(FaceElement),
	Image(ImageElement<'e>),
	File(FileElement<'e>),
	Video(VideoElement<'e>),
	Record(RecordElement<'e>),
	Json(JsonElement<'e>),
	Xml(XmlElement<'e>),
}

impl<'e> Elements<'e> {
	/// 若为文本元素，返回文本内容。
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(element) => Some(element.text),
			_ => None,
		}
	}

	/// 若为 At 元素，返回元素引用。
	pub fn as_at(&self) -> Option<&AtElement<'_>> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	/// 若为回复元素，返回元素引用。
	pub fn as_reply(&self) -> Option<&ReplyElement<'_>> {
		match self {
			Elements::Reply(element) => Some(element),
			_ => None,
		}
	}

	/// 若为表情元素，返回元素引用。
	pub fn as_face(&self) -> Option<&FaceElement> {
		match self {
			Elements::Face(element) => Some(element),
			_ => None,
		}
	}

	/// 若为图片元素，返回元素引用。
	pub fn as_image(&self) -> Option<&ImageElement<'_>> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	/// 若为文件元素，返回元素引用。
	pub fn as_file(&self) -> Option<&FileElement<'_>> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	/// 若为视频元素，返回元素引用。
	pub fn as_video(&self) -> Option<&VideoElement<'_>> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	/// 若为语音元素，返回元素引用。
	pub fn as_record(&self) -> Option<&RecordElement<'_>> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 JSON 元素，返回元素引用。
	pub fn as_json(&self) -> Option<&JsonElement<'_>> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 XML 元素，返回元素引用。
	pub fn as_xml(&self) -> Option<&XmlElement<'_>> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}

impl<'e> Element for Elements<'e> {
	fn r#type(&self) -> ElementType {
		match self {
			Elements::Text(element) => element.r#type(),
			Elements::File(element) => element.r#type(),
			Elements::Face(element) => element.r#type(),
			Elements::Image(element) => element.r#type(),
			Elements::Json(element) => element.r#type(),
			Elements::Record(element) => element.r#type(),
			Elements::Reply(element) => element.r#type(),
			Elements::Video(element) => element.r#type(),
			Elements::Xml(element) => element.r#type(),
			Elements::At(element) => element.r#type(),
		}
	}
}
