use bytes::Bytes;
use puniyu_element::send::Elements;
use puniyu_segment::{Segment, segment};

#[test]
fn test_mixed_usage_method_and_macro() {
	let text1 = Segment::text("method created");
	let text2 = segment!(text, "macro created");

	assert_eq!(text1.as_text(), Some("method created"));
	assert_eq!(text2.as_text(), Some("macro created"));
}

#[test]
fn test_message_chain() {
	let message: Vec<Elements> = vec![
		segment!(at, "123456"),
		segment!(text, " Hello!"),
		segment!(face, 178u64),
		segment!(text, " This is a test message"),
	];

	assert_eq!(message.len(), 4);
	assert_eq!(message[0].as_at().unwrap().target_id, "123456");
	assert_eq!(message[1].as_text(), Some(" Hello!"));
	assert_eq!(message[2].as_face().unwrap().id, 178);
	assert_eq!(message[3].as_text(), Some(" This is a test message"));
}

#[test]
fn test_reply_with_content() {
	let message: Vec<Elements> =
		vec![segment!(reply, "original_msg_id"), segment!(text, "This is reply content")];

	assert_eq!(message.len(), 2);
	assert_eq!(message[0].as_reply().unwrap().message_id, "original_msg_id");
	assert_eq!(message[1].as_text(), Some("This is reply content"));
}

#[test]
fn test_image_message_with_text() {
	let image_data = Bytes::from("fake image");
	let message: Vec<Elements> = vec![
		segment!(text, "Check out this image:"),
		segment!(image, image_data, "photo.jpg", "Beautiful scenery"),
	];

	assert_eq!(message.len(), 2);
	assert_eq!(message[0].as_text(), Some("Check out this image:"));
	assert!(message[1].as_image().is_some());
}

#[test]
fn test_at_all_with_announcement() {
	let message: Vec<Elements> =
		vec![segment!(at, all), segment!(text, " Important notice: System maintenance tonight")];

	assert_eq!(message.len(), 2);
	assert_eq!(message[0].as_at().unwrap().target_id, "all");
}

#[test]
fn test_multiple_at_users() {
	let users = vec!["user1", "user2", "user3"];
	let mut message: Vec<Elements> = Vec::new();

	for user in users {
		message.push(segment!(at, user));
		message.push(segment!(text, " "));
	}
	message.push(segment!(text, "Hello everyone!"));

	assert_eq!(message.len(), 7);
}

#[test]
fn test_file_with_description() {
	let file_data = Bytes::from("document content");
	let message: Vec<Elements> =
		vec![segment!(text, "Here is today's report:"), segment!(file, file_data, "report.pdf")];

	assert_eq!(message.len(), 2);
	assert_eq!(message[1].as_file().unwrap().file_name, "report.pdf");
}

#[test]
fn test_mixed_media_message() {
	let image_data = Bytes::from("image");
	let video_data = Bytes::from("video");
	let audio_data = Bytes::from("audio");

	let message: Vec<Elements> = vec![
		segment!(text, "Multimedia message:"),
		segment!(image, image_data, "pic.jpg"),
		segment!(video, video_data, "clip.mp4"),
		segment!(record, audio_data, "voice.mp3"),
	];

	assert_eq!(message.len(), 4);
	assert!(message[1].as_image().is_some());
	assert!(message[2].as_video().is_some());
	assert!(message[3].as_record().is_some());
}

#[test]
fn test_json_card_message() {
	let json_card = r#"{
		"app": "com.tencent.miniapp",
		"desc": "Mini Program",
		"view": "notification"
	}"#;

	let message: Vec<Elements> =
		vec![segment!(text, "Share a mini program:"), segment!(json, json_card)];

	assert_eq!(message.len(), 2);
	assert!(message[1].as_json().is_some());
}

#[test]
fn test_xml_card_message() {
	let xml_card = r#"<msg><item>Card message</item></msg>"#;
	let message: Vec<Elements> = vec![segment!(xml, xml_card)];

	assert_eq!(message.len(), 1);
	assert!(message[0].as_xml().is_some());
}

#[test]
fn test_empty_message_chain() {
	let message: Vec<Elements> = Vec::new();
	assert_eq!(message.len(), 0);
}

#[test]
fn test_single_element_message() {
	let message: Vec<Elements> = vec![segment!(text, "Single message")];
	assert_eq!(message.len(), 1);
	assert_eq!(message[0].as_text(), Some("Single message"));
}

#[test]
fn test_elements_as_methods() {
	let text_elem = segment!(text, "test");
	assert_eq!(text_elem.as_text(), Some("test"));
	assert!(text_elem.as_at().is_none());
	assert!(text_elem.as_face().is_none());

	let at_elem = segment!(at, "user");
	assert!(at_elem.as_at().is_some());
	assert!(at_elem.as_text().is_none());
}

#[test]
fn test_complex_message_scenario() {
	let image_data = Bytes::from("image data");

	let message: Vec<Elements> = vec![
		segment!(reply, "msg_123"),
		segment!(at, "user_456"),
		segment!(text, " You're right!"),
		segment!(face, 32u64),
		segment!(text, " Check this image:"),
		segment!(image, image_data, "example.jpg", "Example image"),
	];

	assert_eq!(message.len(), 6);
	assert!(message[0].as_reply().is_some());
	assert!(message[1].as_at().is_some());
	assert!(message[2].as_text().is_some());
	assert!(message[3].as_face().is_some());
	assert!(message[4].as_text().is_some());
	assert!(message[5].as_image().is_some());
}

#[test]
fn test_method_and_macro_equivalence() {
	let text_method = Segment::text("test");
	let text_macro = segment!(text, "test");

	assert_eq!(text_method.as_text(), text_macro.as_text());

	let face_method = Segment::face(123u64);
	let face_macro = segment!(face, 123u64);

	assert_eq!(face_method.as_face().unwrap().id, face_macro.as_face().unwrap().id);
}

#[test]
fn test_builder_pattern_simulation() {
	fn build_greeting_message(user_id: &str, name: &str) -> Vec<Elements> {
		vec![
			segment!(at, user_id),
			segment!(text, " Hello, "),
			segment!(text, name),
			segment!(text, "!"),
			segment!(face, 178u64),
		]
	}

	let message = build_greeting_message("123", "Alice");
	assert_eq!(message.len(), 5);
	assert_eq!(message[0].as_at().unwrap().target_id, "123");
}

#[test]
fn test_conditional_message_building() {
	let include_image = true;
	let mut message: Vec<Elements> = vec![segment!(text, "Message content")];

	if include_image {
		let image_data = Bytes::from("image");
		message.push(segment!(image, image_data, "pic.jpg"));
	}

	assert_eq!(message.len(), 2);
	assert!(message[1].as_image().is_some());
}

#[test]
fn test_message_filtering() {
	let message: Vec<Elements> = vec![
		segment!(text, "text1"),
		segment!(at, "user"),
		segment!(text, "text2"),
		segment!(face, 123u64),
	];

	let text_count = message.iter().filter(|e| e.as_text().is_some()).count();
	assert_eq!(text_count, 2);

	let at_count = message.iter().filter(|e| e.as_at().is_some()).count();
	assert_eq!(at_count, 1);
}

#[test]
fn test_message_transformation() {
	let message: Vec<Elements> = vec![segment!(text, "hello"), segment!(text, "world")];

	let texts: Vec<&str> = message.iter().filter_map(|e| e.as_text()).collect();

	assert_eq!(texts, vec!["hello", "world"]);
}
