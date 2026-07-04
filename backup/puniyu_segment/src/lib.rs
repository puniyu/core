//! # puniyu_segment
//!
//! 消息段构建库，统一生成 `Elements`。
//!
//! ## 特性
//!
//! - `Segment` 方法构建
//! - `segment!` 宏构建
//! - 可直接组合 `Vec<Elements>`
//!
//! ```rust
//! use puniyu_segment::segment;
//! use puniyu_element::send::Elements;
//!
//! let message: Vec<Elements> = vec![
//!     segment!(at, "123456"),
//!     segment!(text, " hello"),
//!     segment!(face, 178u64),
//! ];
//! ```

use bytes::Bytes;
use puniyu_element::send::{
	AtElement, Elements, FaceElement, FileElement, ImageElement, JsonElement, RecordElement,
	ReplyElement, TextElement, VideoElement, XmlElement,
};

mod macros;

/// 消息段构建器。
pub struct Segment;

impl Segment {
	/// 创建 At 元素。
	pub fn at(target_id: impl Into<String>) -> Elements {
		let elem = AtElement::new(target_id);
		Elements::At(elem)
	}

	/// 创建文本元素。
	pub fn text(text: impl Into<String>) -> Elements {
		let elem = TextElement::new(text);
		Elements::Text(elem)
	}

	/// 创建表情元素。
	pub fn face(id: impl Into<u64>) -> Elements {
		let elem = FaceElement::new(id.into());
		Elements::Face(elem)
	}

	/// 创建回复元素。
	pub fn reply(message_id: impl Into<String>) -> Elements {
		let elem = ReplyElement::new(message_id);
		Elements::Reply(elem)
	}

	/// 创建图片元素。
	pub fn image(
		image: impl Into<Bytes>,
		name: impl Into<String>,
		summary: Option<String>,
	) -> Elements {
		let elem = ImageElement::new(image, name, summary);
		Elements::Image(elem)
	}

	/// 创建语音元素。
	pub fn record(record: impl Into<Bytes>, name: impl Into<String>) -> Elements {
		let elem = RecordElement::new(record.into(), name);
		Elements::Record(elem)
	}

	/// 创建文件元素。
	pub fn file(file: impl Into<Bytes>, name: impl Into<String>) -> Elements {
		let elem = FileElement::new(file.into(), name);
		Elements::File(elem)
	}

	/// 创建视频元素。
	pub fn video(video: impl Into<Bytes>, name: impl Into<String>) -> Elements {
		let elem = VideoElement::new(video.into(), name);
		Elements::Video(elem)
	}

	/// 创建 JSON 元素。
	pub fn json(json: impl Into<String>) -> Elements {
		let elem = JsonElement::new(json);
		Elements::Json(elem)
	}

	/// 创建 XML 元素。
	pub fn xml(xml: impl Into<String>) -> Elements {
		let elem = XmlElement::new(xml);
		Elements::Xml(elem)
	}
}
