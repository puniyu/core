//! 发送消息元素模块
//!
//! 用于构造发送消息元素。

use crate::{Element, ElementType, codegen_reexport};
use serde::{Deserialize, Serialize};
use strum::Display;

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

/// 发送元素枚举。
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum Elements {
	Text(TextElement),
	At(AtElement),
	Reply(ReplyElement),
	Face(FaceElement),
	Image(ImageElement),
	File(FileElement),
	Video(VideoElement),
	Record(RecordElement),
	Json(JsonElement),
	Xml(XmlElement),
}

impl Elements {
	/// 若为文本元素，返回文本内容。
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(element) => Some(element.text.as_str()),
			_ => None,
		}
	}

	/// 若为 At 元素，返回元素引用。
	pub fn as_at(&self) -> Option<&AtElement> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	/// 若为回复元素，返回元素引用。
	pub fn as_reply(&self) -> Option<&ReplyElement> {
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
	pub fn as_image(&self) -> Option<&ImageElement> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	/// 若为文件元素，返回元素引用。
	pub fn as_file(&self) -> Option<&FileElement> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	/// 若为视频元素，返回元素引用。
	pub fn as_video(&self) -> Option<&VideoElement> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	/// 若为语音元素，返回元素引用。
	pub fn as_record(&self) -> Option<&RecordElement> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 JSON 元素，返回元素引用。
	pub fn as_json(&self) -> Option<&JsonElement> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	/// 若为 XML 元素，返回元素引用。
	pub fn as_xml(&self) -> Option<&XmlElement> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}

impl Element for Elements {
	fn r#type(&self) -> ElementType {
		match self {
			Elements::Text(element) => element.r#type(),
			Elements::At(element) => element.r#type(),
			Elements::Reply(element) => element.r#type(),
			Elements::Face(element) => element.r#type(),
			Elements::Image(element) => element.r#type(),
			Elements::File(element) => element.r#type(),
			Elements::Video(element) => element.r#type(),
			Elements::Record(element) => element.r#type(),
			Elements::Json(element) => element.r#type(),
			Elements::Xml(element) => element.r#type(),
		}
	}
}

impl From<TextElement> for Elements {
	fn from(element: TextElement) -> Self {
		Self::Text(element)
	}
}

impl From<AtElement> for Elements {
	fn from(element: AtElement) -> Self {
		Self::At(element)
	}
}

impl From<ReplyElement> for Elements {
	fn from(element: ReplyElement) -> Self {
		Self::Reply(element)
	}
}

impl From<FaceElement> for Elements {
	fn from(element: FaceElement) -> Self {
		Self::Face(element)
	}
}

impl From<ImageElement> for Elements {
	fn from(element: ImageElement) -> Self {
		Self::Image(element)
	}
}

impl From<FileElement> for Elements {
	fn from(element: FileElement) -> Self {
		Self::File(element)
	}
}

impl From<VideoElement> for Elements {
	fn from(element: VideoElement) -> Self {
		Self::Video(element)
	}
}

impl From<RecordElement> for Elements {
	fn from(element: RecordElement) -> Self {
		Self::Record(element)
	}
}

impl From<JsonElement> for Elements {
	fn from(element: JsonElement) -> Self {
		Self::Json(element)
	}
}

impl From<XmlElement> for Elements {
	fn from(element: XmlElement) -> Self {
		Self::Xml(element)
	}
}
