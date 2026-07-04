use bytes::Bytes;
use puniyu_element::{receive, send};

#[test]
fn test_receive_text_serialization() {
	let element = receive::TextElement { text: "test text" };
	let json = serde_json::to_string(&element).unwrap();

	assert!(json.contains("test text"));

	let deserialized: receive::TextElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.text, "test text");
}

#[test]
fn test_receive_at_serialization() {
	let element = receive::AtElement { target_id: "123456" };
	let json = serde_json::to_string(&element).unwrap();

	let deserialized: receive::AtElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.target_id, "123456");
}

#[test]
fn test_receive_face_serialization() {
	let element = receive::FaceElement { id: 42 };
	let json = serde_json::to_string(&element).unwrap();

	let deserialized: receive::FaceElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.id, 42);
}

#[test]
fn test_receive_json_serialization() {
	let json_data = r#"{"key": "value"}"#;
	let element = receive::JsonElement { data: json_data };
	let json = serde_json::to_string(&element).unwrap();
	assert!(json.contains("key"));
}

#[test]
fn test_send_text_serialization() {
	let element = send::TextElement::new("send test");
	let json = serde_json::to_string(&element).unwrap();

	let deserialized: send::TextElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.text, "send test");
}

#[test]
fn test_send_at_serialization() {
	let element = send::AtElement::new("target_user");
	let json = serde_json::to_string(&element).unwrap();

	let deserialized: send::AtElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.target_id, "target_user");
}

#[test]
fn test_send_face_serialization() {
	let element = send::FaceElement::new(99u64);
	let json = serde_json::to_string(&element).unwrap();

	let deserialized: send::FaceElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.id, 99);
}

#[test]
fn test_receive_elements_enum_serialization() {
	let text_elem = receive::TextElement { text: "enum test" };
	let element = receive::Elements::Text(text_elem);

	let json = serde_json::to_string(&element).unwrap();
	assert!(json.contains("text"));
	assert!(json.contains("enum test"));

	let deserialized: receive::Elements = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.as_text(), Some("enum test"));
}

#[test]
fn test_send_elements_enum_serialization() {
	let text_elem = send::TextElement::new("enum send");
	let element = send::Elements::Text(text_elem);

	let json = serde_json::to_string(&element).unwrap();
	assert!(json.contains("text"));
	assert!(json.contains("enum send"));

	let deserialized: send::Elements = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.as_text(), Some("enum send"));
}

#[test]
fn test_mixed_elements_serialization() {
	let elements = vec![
		receive::Elements::Text(receive::TextElement { text: "hello" }),
		receive::Elements::At(receive::AtElement { target_id: "user1" }),
		receive::Elements::Face(receive::FaceElement { id: 1 }),
	];

	let json = serde_json::to_string(&elements).unwrap();
	let deserialized: Vec<receive::Elements> = serde_json::from_str(&json).unwrap();

	assert_eq!(deserialized.len(), 3);
	assert_eq!(deserialized[0].as_text(), Some("hello"));
	assert!(deserialized[1].as_at().is_some());
	assert!(deserialized[2].as_face().is_some());
}

#[test]
fn test_bytes_handling() {
	let data = vec![1, 2, 3, 4, 5];
	let bytes_data = Bytes::from(data.clone());

	let element = send::FileElement::new(bytes_data.clone(), "test.bin");

	assert_eq!(element.file.len(), 5);
	assert_eq!(element.file.as_ref(), data.as_slice());
	assert_eq!(element.file_name, "test.bin");
}

#[test]
fn test_empty_bytes() {
	let empty_data: Vec<u8> = vec![];
	let element = send::ImageElement::new(empty_data, "empty.png", None);

	assert_eq!(element.file.len(), 0);
	assert_eq!(element.summary, "empty.png");
}

#[test]
fn test_text_element_conversions() {
	let text = "convert test";
	let element = receive::TextElement::from(text);

	let as_str: &str = element.clone().into();
	assert_eq!(as_str, text);

	let as_string: String = element.into();
	assert_eq!(as_string, text);
}

#[test]
fn test_at_element_conversions() {
	let target = "user123";
	let element = receive::AtElement::from(target);

	let as_str: &str = element.clone().into();
	assert_eq!(as_str, target);

	let as_string: String = element.into();
	assert_eq!(as_string, target);
}

#[test]
fn test_face_element_from_string() {
	let element = receive::FaceElement::from("42");
	assert_eq!(element.id, 42);

	let invalid = receive::FaceElement::from("invalid");
	assert_eq!(invalid.id, 0);
}

#[test]
fn test_empty_string_elements() {
	let text = receive::TextElement { text: "" };
	assert_eq!(text.text, "");

	let json = receive::JsonElement { data: "" };
	assert_eq!(json.data, "");
}

#[test]
fn test_unicode_content() {
	let text = receive::TextElement { text: "hello world" };
	assert_eq!(text.text, "hello world");

	let json = serde_json::to_string(&text).unwrap();
	let deserialized: receive::TextElement = serde_json::from_str(&json).unwrap();
	assert_eq!(deserialized.text, "hello world");
}

#[test]
fn test_large_bytes_data() {
	let large_data = vec![0u8; 10000];
	let element = send::FileElement::new(large_data.clone(), "large.bin");

	assert_eq!(element.file.len(), 10000);
	assert_eq!(element.file.as_ref(), large_data.as_slice());
}
