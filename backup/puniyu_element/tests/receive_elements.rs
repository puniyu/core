use bytes::Bytes;
use puniyu_element::receive::*;
use puniyu_element::{Element, ElementType};

#[test]
fn test_text_element_creation() {
	let text = "test text";
	let element = TextElement { text };

	assert_eq!(element.text, "test text");
	assert_eq!(element.r#type(), ElementType::Text);
}

#[test]
fn test_text_element_from_str() {
	let text = "Hello World";
	let element = TextElement::from(text);

	assert_eq!(element.text, text);
}

#[test]
fn test_text_element_to_string() {
	let element = TextElement { text: "convert test" };
	let result: String = element.clone().into();

	assert_eq!(result, "convert test");
}

#[test]
fn test_text_element_to_str() {
	let element = TextElement { text: "reference test" };
	let result: &str = element.into();

	assert_eq!(result, "reference test");
}

#[test]
fn test_at_element_creation() {
	let element = AtElement { target_id: "123456" };

	assert_eq!(element.target_id, "123456");
	assert_eq!(element.r#type(), ElementType::At);
}

#[test]
fn test_at_element_is_everyone() {
	let everyone = AtElement { target_id: "all" };
	let specific = AtElement { target_id: "123456" };

	assert!(everyone.is_everyone());
	assert!(!specific.is_everyone());
}

#[test]
fn test_at_element_from_str() {
	let element = AtElement::from("789012");

	assert_eq!(element.target_id, "789012");
}

#[test]
fn test_at_element_conversions() {
	let element = AtElement { target_id: "user123" };

	let as_str: &str = element.clone().into();
	assert_eq!(as_str, "user123");

	let as_string: String = element.into();
	assert_eq!(as_string, "user123");
}

#[test]
fn test_reply_element_creation() {
	let element = ReplyElement { message_id: "msg_001" };

	assert_eq!(element.message_id, "msg_001");
	assert_eq!(element.r#type(), ElementType::Reply);
}

#[test]
fn test_reply_element_from_str() {
	let element = ReplyElement::from("msg_002");

	assert_eq!(element.message_id, "msg_002");
}

#[test]
fn test_reply_element_to_string() {
	let element = ReplyElement { message_id: "msg_003" };
	let result: String = element.into();

	assert_eq!(result, "msg_003");
}

#[test]
fn test_face_element_creation() {
	let element = FaceElement { id: 42 };

	assert_eq!(element.id, 42);
	assert_eq!(element.r#type(), ElementType::Face);
}

#[test]
fn test_face_element_from_str() {
	let element = FaceElement::from("99");

	assert_eq!(element.id, 99);
}

#[test]
fn test_face_element_to_string() {
	let element = FaceElement { id: 10 };
	let result: String = element.into();

	assert_eq!(result, "10");
}

#[test]
fn test_image_element_creation() {
	let file_data = Bytes::from_static(b"image data");
	let element = ImageElement {
		file: file_data.clone(),
		file_name: "image.jpg",
		summary: "test image",
		width: 800,
		height: 600,
	};

	assert_eq!(element.file, file_data);
	assert_eq!(element.file_name, "image.jpg");
	assert_eq!(element.summary, "test image");
	assert_eq!(element.width, 800);
	assert_eq!(element.height, 600);
	assert_eq!(element.r#type(), ElementType::Image);
}

#[test]
fn test_file_element_creation() {
	let file_data = Bytes::from_static(b"file content");
	let element =
		FileElement { file: file_data.clone(), file_size: 1024, file_name: "document.pdf" };

	assert_eq!(element.file, file_data);
	assert_eq!(element.file_size, 1024);
	assert_eq!(element.file_name, "document.pdf");
	assert_eq!(element.r#type(), ElementType::File);
}

#[test]
fn test_video_element_creation() {
	let video_data = Bytes::from_static(b"video data");
	let element = VideoElement { file: video_data.clone(), file_name: "video.mp4" };

	assert_eq!(element.file, video_data);
	assert_eq!(element.file_name, "video.mp4");
	assert_eq!(element.r#type(), ElementType::Video);
}

#[test]
fn test_record_element_creation() {
	let audio_data = Bytes::from_static(b"audio data");
	let element = RecordElement { file: audio_data.clone(), file_name: "audio.mp3" };

	assert_eq!(element.file, audio_data);
	assert_eq!(element.file_name, "audio.mp3");
	assert_eq!(element.r#type(), ElementType::Record);
}

#[test]
fn test_json_element_creation() {
	let json_data = r#"{"key": "value"}"#;
	let element = JsonElement { data: json_data };

	assert_eq!(element.data, json_data);
	assert_eq!(element.r#type(), ElementType::Json);
}

#[test]
fn test_json_element_conversions() {
	let json_data = r#"{"test": true}"#;
	let element = JsonElement { data: json_data };

	let as_str: &str = element.clone().into();
	assert_eq!(as_str, json_data);

	let as_string: String = element.into();
	assert_eq!(as_string, json_data);
}

#[test]
fn test_xml_element_creation() {
	let xml_data = "<root><item>test</item></root>";
	let element = XmlElement { data: xml_data };

	assert_eq!(element.data, xml_data);
	assert_eq!(element.r#type(), ElementType::Xml);
}

#[test]
fn test_xml_element_conversions() {
	let xml_data = "<xml/>";
	let element = XmlElement { data: xml_data };

	let as_str: &str = element.clone().into();
	assert_eq!(as_str, xml_data);

	let as_string: String = element.into();
	assert_eq!(as_string, xml_data);
}

#[test]
fn test_elements_enum_text() {
	let text_elem = TextElement { text: "test" };
	let element = Elements::Text(text_elem);

	assert_eq!(element.as_text(), Some("test"));
	assert!(element.as_at().is_none());
	assert_eq!(element.r#type(), ElementType::Text);
}

#[test]
fn test_elements_enum_at() {
	let at_elem = AtElement { target_id: "user" };
	let element = Elements::At(at_elem);

	assert!(element.as_text().is_none());
	assert!(element.as_at().is_some());
	assert_eq!(element.as_at().unwrap().target_id, "user");
	assert_eq!(element.r#type(), ElementType::At);
}

#[test]
fn test_elements_enum_reply() {
	let reply_elem = ReplyElement { message_id: "msg" };
	let element = Elements::Reply(reply_elem);

	assert!(element.as_reply().is_some());
	assert_eq!(element.as_reply().unwrap().message_id, "msg");
}

#[test]
fn test_elements_enum_face() {
	let face_elem = FaceElement { id: 1 };
	let element = Elements::Face(face_elem);

	assert!(element.as_face().is_some());
	assert_eq!(element.as_face().unwrap().id, 1);
}

#[test]
fn test_elements_enum_image() {
	let file_data = Bytes::from_static(b"img");
	let image_elem = ImageElement {
		file: file_data,
		file_name: "img.jpg",
		summary: "image",
		width: 100,
		height: 100,
	};
	let element = Elements::Image(image_elem);

	assert!(element.as_image().is_some());
	assert_eq!(element.as_image().unwrap().file_name, "img.jpg");
}

#[test]
fn test_elements_enum_file() {
	let file_data = Bytes::from_static(b"data");
	let file_elem = FileElement { file: file_data, file_size: 100, file_name: "test.txt" };
	let element = Elements::File(file_elem);

	assert!(element.as_file().is_some());
	assert_eq!(element.as_file().unwrap().file_name, "test.txt");
}

#[test]
fn test_elements_enum_video() {
	let video_data = Bytes::from_static(b"vid");
	let video_elem = VideoElement { file: video_data, file_name: "vid.mp4" };
	let element = Elements::Video(video_elem);

	assert!(element.as_video().is_some());
	assert_eq!(element.as_video().unwrap().file_name, "vid.mp4");
}

#[test]
fn test_elements_enum_record() {
	let audio_data = Bytes::from_static(b"audio");
	let record_elem = RecordElement { file: audio_data, file_name: "audio.mp3" };
	let element = Elements::Record(record_elem);

	assert!(element.as_record().is_some());
	assert_eq!(element.as_record().unwrap().file_name, "audio.mp3");
}

#[test]
fn test_elements_enum_json() {
	let json_elem = JsonElement { data: "{}" };
	let element = Elements::Json(json_elem);

	assert!(element.as_json().is_some());
	assert_eq!(element.as_json().unwrap().data, "{}");
}

#[test]
fn test_elements_enum_xml() {
	let xml_elem = XmlElement { data: "<xml/>" };
	let element = Elements::Xml(xml_elem);

	assert!(element.as_xml().is_some());
	assert_eq!(element.as_xml().unwrap().data, "<xml/>");
}
