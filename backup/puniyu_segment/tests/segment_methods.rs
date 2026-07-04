use bytes::Bytes;
use puniyu_segment::Segment;

#[test]
fn test_segment_text() {
	let element = Segment::text("Hello, World!");
	assert_eq!(element.as_text(), Some("Hello, World!"));
}

#[test]
fn test_segment_at() {
	let element = Segment::at("123456");
	let at_elem = element.as_at().expect("Expected At element");
	assert_eq!(at_elem.target_id, "123456");
}

#[test]
fn test_segment_at_all() {
	let element = Segment::at("all");
	let at_elem = element.as_at().expect("Expected At element");
	assert_eq!(at_elem.target_id, "all");
}

#[test]
fn test_segment_face() {
	let element = Segment::face(178u64);
	let face_elem = element.as_face().expect("Expected Face element");
	assert_eq!(face_elem.id, 178);
}

#[test]
fn test_segment_face_from_u32() {
	let element = Segment::face(42u32);
	let face_elem = element.as_face().expect("Expected Face element");
	assert_eq!(face_elem.id, 42);
}

#[test]
fn test_segment_reply() {
	let element = Segment::reply("message_id_123");
	let reply_elem = element.as_reply().expect("Expected Reply element");
	assert_eq!(reply_elem.message_id, "message_id_123");
}

#[test]
fn test_segment_image_without_summary() {
	let image_data = Bytes::from("fake image data");
	let element = Segment::image(image_data.clone(), "photo.jpg", None);
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file, image_data);
	assert_eq!(image_elem.file_name, "photo.jpg");
	assert_eq!(image_elem.summary, "photo.jpg");
}

#[test]
fn test_segment_image_with_summary() {
	let image_data = Bytes::from("fake image data");
	let element = Segment::image(image_data.clone(), "photo.jpg", Some("Beautiful scenery".into()));
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file, image_data);
	assert_eq!(image_elem.file_name, "photo.jpg");
	assert_eq!(image_elem.summary, "Beautiful scenery");
}

#[test]
fn test_segment_file() {
	let file_data = Bytes::from("file content");
	let element = Segment::file(file_data.clone(), "document.pdf");
	let file_elem = element.as_file().expect("Expected File element");

	assert_eq!(file_elem.file, file_data);
	assert_eq!(file_elem.file_name, "document.pdf");
}

#[test]
fn test_segment_record() {
	let audio_data = Bytes::from("audio content");
	let element = Segment::record(audio_data.clone(), "voice.mp3");
	let record_elem = element.as_record().expect("Expected Record element");

	assert_eq!(record_elem.file, audio_data);
	assert_eq!(record_elem.file_name, "voice.mp3");
}

#[test]
fn test_segment_video() {
	let video_data = Bytes::from("video content");
	let element = Segment::video(video_data.clone(), "clip.mp4");
	let video_elem = element.as_video().expect("Expected Video element");

	assert_eq!(video_elem.file, video_data);
	assert_eq!(video_elem.file_name, "clip.mp4");
}

#[test]
fn test_segment_json() {
	let json_data = r#"{"key": "value"}"#;
	let element = Segment::json(json_data);
	let json_elem = element.as_json().expect("Expected Json element");
	assert_eq!(json_elem.data, json_data);
}

#[test]
fn test_segment_xml() {
	let xml_data = r#"<xml><item>value</item></xml>"#;
	let element = Segment::xml(xml_data);
	let xml_elem = element.as_xml().expect("Expected Xml element");
	assert_eq!(xml_elem.data, xml_data);
}

#[test]
fn test_segment_empty_text() {
	let element = Segment::text("");
	assert_eq!(element.as_text(), Some(""));
}

#[test]
fn test_segment_empty_bytes() {
	let empty_data = Bytes::new();
	let element = Segment::image(empty_data, "empty.png", None);
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file.len(), 0);
	assert_eq!(image_elem.file_name, "empty.png");
}

#[test]
fn test_segment_bytes_from_vec() {
	let data = vec![1u8, 2, 3, 4, 5];
	let element = Segment::file(data.clone(), "test.bin");
	let file_elem = element.as_file().expect("Expected File element");
	assert_eq!(file_elem.file.as_ref(), data.as_slice());
}

#[test]
fn test_segment_unicode_text() {
	let element = Segment::text("Hello🎉World🌍");
	assert_eq!(element.as_text(), Some("Hello🎉World🌍"));
}

#[test]
fn test_segment_special_characters() {
	let element = Segment::text("Special chars: \n\t\r\\\"'");
	assert_eq!(element.as_text(), Some("Special chars: \n\t\r\\\"'"));
}

#[test]
fn test_segment_text_from_string() {
	let element = Segment::text(String::from("owned text"));
	let text_elem = element.as_text();

	assert_eq!(text_elem, Some("owned text"));
}

#[test]
fn test_segment_image_from_owned_strings() {
	let element = Segment::image(
		Bytes::from_static(b"image"),
		String::from("owned.png"),
		Some(String::from("owned summary")),
	);
	let image_elem = element.as_image().expect("Expected Image element");

	assert_eq!(image_elem.file_name, "owned.png");
	assert_eq!(image_elem.summary, "owned summary");
}

#[test]
fn test_segment_type_checking() {
	let text = Segment::text("test");
	assert!(text.as_text().is_some());
	assert!(text.as_at().is_none());
	assert!(text.as_face().is_none());

	let at = Segment::at("user");
	assert!(at.as_at().is_some());
	assert!(at.as_text().is_none());
}
