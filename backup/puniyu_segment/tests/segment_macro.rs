use bytes::Bytes;
use puniyu_segment::segment;

#[test]
fn test_macro_text() {
	let element = segment!(text, "Hello, World!");
	assert_eq!(element.as_text(), Some("Hello, World!"));
}

#[test]
fn test_macro_at() {
	let element = segment!(at, "123456");
	let at_elem = element.as_at().expect("Expected At element");
	assert_eq!(at_elem.target_id, "123456");
}

#[test]
fn test_macro_at_all() {
	let element = segment!(at, all);
	let at_elem = element.as_at().expect("Expected At element");
	assert_eq!(at_elem.target_id, "all");
}

#[test]
fn test_macro_face() {
	let element = segment!(face, 178u64);
	let face_elem = element.as_face().expect("Expected Face element");
	assert_eq!(face_elem.id, 178);
}

#[test]
fn test_macro_reply() {
	let element = segment!(reply, "message_id_123");
	let reply_elem = element.as_reply().expect("Expected Reply element");
	assert_eq!(reply_elem.message_id, "message_id_123");
}

#[test]
fn test_macro_image_without_summary() {
	let image_data = Bytes::from("fake image data");
	let element = segment!(image, image_data.clone(), "photo.jpg");
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file, image_data);
	assert_eq!(image_elem.file_name, "photo.jpg");
	assert_eq!(image_elem.summary, "photo.jpg");
}

#[test]
fn test_macro_image_with_summary() {
	let image_data = Bytes::from("fake image data");
	let element = segment!(image, image_data.clone(), "photo.jpg", "Beautiful scenery");
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file, image_data);
	assert_eq!(image_elem.file_name, "photo.jpg");
	assert_eq!(image_elem.summary, "Beautiful scenery");
}

#[test]
fn test_macro_file() {
	let file_data = Bytes::from("file content");
	let element = segment!(file, file_data.clone(), "document.pdf");
	let file_elem = element.as_file().expect("Expected File element");

	assert_eq!(file_elem.file, file_data);
	assert_eq!(file_elem.file_name, "document.pdf");
}

#[test]
fn test_macro_record() {
	let audio_data = Bytes::from("audio content");
	let element = segment!(record, audio_data.clone(), "voice.mp3");
	let record_elem = element.as_record().expect("Expected Record element");

	assert_eq!(record_elem.file, audio_data);
	assert_eq!(record_elem.file_name, "voice.mp3");
}

#[test]
fn test_macro_video() {
	let video_data = Bytes::from("video content");
	let element = segment!(video, video_data.clone(), "clip.mp4");
	let video_elem = element.as_video().expect("Expected Video element");

	assert_eq!(video_elem.file, video_data);
	assert_eq!(video_elem.file_name, "clip.mp4");
}

#[test]
fn test_macro_json() {
	let json_data = r#"{"key": "value"}"#;
	let element = segment!(json, json_data);
	let json_elem = element.as_json().expect("Expected Json element");
	assert_eq!(json_elem.data, json_data);
}

#[test]
fn test_macro_xml() {
	let xml_data = r#"<xml><item>value</item></xml>"#;
	let element = segment!(xml, xml_data);
	let xml_elem = element.as_xml().expect("Expected Xml element");
	assert_eq!(xml_elem.data, xml_data);
}

#[test]
fn test_macro_with_variables() {
	let user_id = "user_123";
	let message = "test message";

	let at_elem = segment!(at, user_id);
	let text_elem = segment!(text, message);

	assert_eq!(at_elem.as_at().unwrap().target_id, "user_123");
	assert_eq!(text_elem.as_text(), Some("test message"));
}

#[test]
fn test_macro_with_expressions() {
	let base_id: u64 = 100;
	let element = segment!(face, base_id + 78);
	let face_elem = element.as_face().expect("Expected Face element");
	assert_eq!(face_elem.id, 178);
}

#[test]
fn test_macro_empty_text() {
	let element = segment!(text, "");
	assert_eq!(element.as_text(), Some(""));
}

#[test]
fn test_macro_unicode_text() {
	let element = segment!(text, "Hello🎉World🌍");
	assert_eq!(element.as_text(), Some("Hello🎉World🌍"));
}

#[test]
fn test_macro_multiline_json() {
	let json = r#"{
		"name": "test",
		"value": 123
	}"#;
	let element = segment!(json, json);
	let json_elem = element.as_json().expect("Expected Json element");

	assert!(json_elem.data.contains("test"));
	assert!(json_elem.data.contains("123"));
}

#[test]
fn test_macro_bytes_from_vec() {
	let data = vec![1u8, 2, 3, 4, 5];
	let element = segment!(file, Bytes::from(data.clone()), "test.bin");
	let file_elem = element.as_file().expect("Expected File element");
	assert_eq!(file_elem.file.as_ref(), data.as_slice());
}

#[test]
fn test_macro_type_checking() {
	let text = segment!(text, "test");
	assert!(text.as_text().is_some());
	assert!(text.as_at().is_none());
	assert!(text.as_face().is_none());

	let at = segment!(at, "user");
	assert!(at.as_at().is_some());
	assert!(at.as_text().is_none());
}

#[test]
fn test_macro_text_from_string_expression() {
	let element = segment!(text, format!("{} {}", "hello", "world"));

	assert_eq!(element.as_text(), Some("hello world"));
}
