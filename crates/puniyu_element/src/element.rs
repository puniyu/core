use super::{RawMessage, TextMessage};
use serde::{Deserialize, Serialize};

/// TODO: 重写raw

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtElement {
	/// at元素目标id
	pub target_id: String,
	/// at元素目标名称
	pub name: Option<String>,
}

impl AtElement {
	/// 是否为艾特全体
	pub fn is_all(&self) -> bool {
		matches!(self.target_id.as_str(), "all")
	}
}

impl RawMessage for AtElement {
	fn raw(&self) -> String {
		format!("[at:{}]", self.target_id)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceElement {
	/// 表情元素id
	pub id: u64,
}

impl RawMessage for FaceElement {
	fn raw(&self) -> String {
		format!("[face:{}]", self.id)
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileElement {
	/// 文件元素
	pub file: Vec<u8>,
	/// 文件id
	pub file_id: String,
	/// 文件大小, 单位字节
	pub file_size: u64,
	/// 文件名称
	pub file_name: String,
}

impl RawMessage for FileElement {
	fn raw(&self) -> String {
		format!("[file:{}]", self.file_id)
	}
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement {
	/// 图片元素
	pub file: Vec<u8>,
	/// 是否为闪照
	pub is_flash: bool,
	/// 图片外显
	pub summary: Option<String>,
}

impl RawMessage for ImageElement {
	fn raw(&self) -> String {
		format!("[image:{}]", self.summary.clone().unwrap_or(String::from("图片")))
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement {
	/// Json数据，未序列化
	pub data: serde_json::Value,
}

impl RawMessage for JsonElement {
	fn raw(&self) -> String {
		format!("[json:{}]", self.data)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement {
	/// 视频元素
	pub file: Vec<u8>,
	/// 视频文件名
	pub file_name: String,
}

impl RawMessage for VideoElement {
	fn raw(&self) -> String {
		format!("[video:{}]", self.file_name)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordElement {
	/// 语言元素
	pub file: Vec<u8>,
}

impl RawMessage for RecordElement {
	fn raw(&self) -> String {
		format!("[record:{}]", "语音消息")
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
	/// 回复元素id
	#[serde(rename = "messageId")]
	pub message_id: String,
}

impl RawMessage for ReplyElement {
	fn raw(&self) -> String {
		format!("[reply:{}]", self.message_id)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
	/// 文本元素内容
	pub text: String,
}

impl RawMessage for TextElement {
	fn raw(&self) -> String {
		format!("[text:{}]", self.text)
	}
}

impl TextMessage for TextElement {
	fn text(&self) -> String {
		self.text.clone()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement {
	/// Xml数据，未序列化
	pub data: String,
}

impl RawMessage for XmlElement {
	fn raw(&self) -> String {
		format!("[xml:{}]", self.data)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Elements {
	At(AtElement),
	File(FileElement),
	Image(ImageElement),
	Json(JsonElement),
	Record(RecordElement),
	Reply(ReplyElement),
	Text(TextElement),
	Video(VideoElement),
	Xml(XmlElement),
}

impl Elements {
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(&text_element.text),
			_ => None,
		}
	}
}

impl RawMessage for Elements {
	fn raw(&self) -> String {
		match self {
			Elements::Text(text_element) => text_element.raw(),
			Elements::File(file_element) => file_element.raw(),
			Elements::Image(image_element) => image_element.raw(),
			Elements::Json(json_element) => json_element.raw(),
			Elements::Record(record_element) => record_element.raw(),
			Elements::Reply(reply_element) => reply_element.raw(),
			Elements::Video(video_element) => video_element.raw(),
			Elements::Xml(xml_element) => xml_element.raw(),
			Elements::At(at_element) => at_element.raw(),
		}
	}
}

impl RawMessage for Vec<Elements> {
	fn raw(&self) -> String {
		self.iter().map(|element| element.raw()).collect::<Vec<_>>().join("")
	}
}
#[macro_export]
macro_rules! element {
	// 文本元素
	(text,$text:expr) => {
		Elements::Text(TextElement {
			r#type: ElementType::Text.to_string(),
			text: $text.to_string(),
		})
	};

	// 图片元素
	(image,$image:expr) => {
		Elements::Image(ImageElement {
			r#type: ElementType::Image.to_string(),
			file: $image.to_string(),
			is_flash: false,
			summary: None,
		})
	};
	(image,$image:expr,$is_flash:expr) => {
		Elements::Image(ImageElement {
			r#type: ElementType::Image.to_string(),
			file: $image.to_string(),
			is_flash: $is_flash,
			summary: None,
		})
	};
	(image,$image:expr,$is_flash:expr,$summary:expr) => {
		Elements::Image(ImageElement {
			r#type: ElementType::Image.to_string(),
			file: $image.to_string(),
			is_flash: $is_flash,
			summary: Some($summary.to_string()),
		})
	};

	// @元素
	(at,$target_id:expr) => {
		Elements::At(AtElement {
			r#type: ElementType::At.to_string(),
			target_id: $target_id.to_string(),
			name: None,
		})
	};

	// @全体成员
	(at_all) => {
		Elements::At(AtElement {
			r#type: ElementType::At.to_string(),
			target_id: "all".to_string(),
			name: Some("全体成员".to_string()),
		})
	};

	// 表情元素
	(face,$id:expr) => {
		Elements::Face(FaceElement { r#type: ElementType::Face.to_string(), id: $id })
	};

	// 回复元素
	(reply,$message_id:expr) => {
		Elements::Reply(ReplyElement {
			r#type: ElementType::Reply.to_string(),
			message_id: $message_id.to_string(),
		})
	};

	// 语音元素
	(record,$record:expr) => {
		Elements::Record(RecordElement {
			r#type: ElementType::Record.to_string(),
			file: $record.to_string(),
		})
	};

	// 文件元素
	(file,$file:expr,$file_id:expr,$file_size:expr,$file_name:expr) => {
		Elements::File(FileElement {
			r#type: ElementType::File.to_string(),
			file: $file.to_string(),
			file_id: $file_id.to_string(),
			file_size: $file_size,
			file_name: $file_name.to_string(),
		})
	};
	// 视频元素
	(video,$video:expr,$video_name:expr) => {
		Elements::Video(VideoElement {
			r#type: ElementType::Video.to_string(),
			file: $video.to_string(),
			file_name: $video_name.to_string(),
		})
	};
	// JSON元素
	(json,$data:expr) => {
		Elements::Json(JsonElement {
			r#type: ElementType::Json.to_string(),
			data: serde_json::Value::String($data.to_string()),
		})
	};

	// XML元素
	(xml,$data:expr) => {
		Elements::Xml(XmlElement {
			r#type: ElementType::Xml.to_string(),
			data: String::from($data),
		})
	};
}
