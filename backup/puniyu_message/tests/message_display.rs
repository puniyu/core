use puniyu_element::send::{AtElement, Elements, FaceElement, TextElement};
use puniyu_message::{Message, message};

#[test]
fn test_display_single_text() {
	let msg = Message::from("Hello, World!");
	let output = format!("{}", msg);

	// Elements::Display 输出枚举变体名称
	assert_eq!(output, "Text");
}

#[test]
fn test_display_multiple_text() {
	let msg = message!(
		Elements::Text(TextElement::new("Hello")),
		Elements::Text(TextElement::new(" ")),
		Elements::Text(TextElement::new("World")),
	);
	let output = format!("{}", msg);

	assert_eq!(output, "TextTextText");
}

#[test]
fn test_display_empty_message() {
	let msg = Message::from(vec![]);
	let output = format!("{}", msg);

	assert_eq!(output, "");
}

#[test]
fn test_display_with_at() {
	let msg = message!(
		Elements::At(AtElement::new("123456")),
		Elements::Text(TextElement::new(" Hello")),
	);
	let output = format!("{}", msg);

	assert_eq!(output, "AtText");
}

#[test]
fn test_display_with_face() {
	let msg = message!(
		Elements::Text(TextElement::new("Happy ")),
		Elements::Face(FaceElement::new(178u64)),
	);
	let output = format!("{}", msg);

	assert_eq!(output, "TextFace");
}

#[test]
fn test_display_alternate_format() {
	let msg = message!(
		Elements::Text(TextElement::new("Line 1")),
		Elements::Text(TextElement::new("Line 2")),
	);
	let output = format!("{:#}", msg);

	assert!(output.contains("\n"));
}

#[test]
fn test_display_alternate_single_element() {
	let msg = Message::from("Single line");
	let output = format!("{:#}", msg);

	assert_eq!(output, "Text");
}

#[test]
fn test_display_complex_message() {
	let msg = message!(
		Elements::At(AtElement::new("all")),
		Elements::Text(TextElement::new(" Notice: ")),
		Elements::Text(TextElement::new("Meeting at 3pm")),
		Elements::Face(FaceElement::new(1u64)),
	);
	let output = format!("{}", msg);

	assert_eq!(output, "AtTextTextFace");
}

#[test]
fn test_display_mixed_elements() {
	let msg = message!(
		Elements::At(AtElement::new("user1")),
		Elements::Face(FaceElement::new(42u64)),
		Elements::Text(TextElement::new("message")),
	);
	let output = format!("{}", msg);

	assert_eq!(output, "AtFaceText");
}

#[test]
fn test_display_only_at_elements() {
	let msg =
		message!(Elements::At(AtElement::new("user1")), Elements::At(AtElement::new("user2")),);
	let output = format!("{}", msg);

	assert_eq!(output, "AtAt");
}

#[test]
fn test_display_only_face_elements() {
	let msg =
		message!(Elements::Face(FaceElement::new(1u64)), Elements::Face(FaceElement::new(2u64)),);
	let output = format!("{}", msg);

	assert_eq!(output, "FaceFace");
}

#[test]
fn test_display_alternate_multiple_elements() {
	let msg = message!(
		Elements::Text(TextElement::new("First")),
		Elements::At(AtElement::new("user")),
		Elements::Face(FaceElement::new(1u64)),
	);
	let output = format!("{:#}", msg);

	// 详细格式每个元素一行
	let lines: Vec<&str> = output.lines().collect();
	assert_eq!(lines.len(), 3);
}

#[test]
fn test_display_debug_format() {
	let msg = Message::from("Test");
	let debug_output = format!("{:?}", msg);

	assert!(debug_output.contains("Message"));
}
