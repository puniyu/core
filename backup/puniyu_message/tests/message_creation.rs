use puniyu_element::send::{AtElement, Elements, FaceElement, TextElement};
use puniyu_message::Message;

#[test]
fn test_message_from_string() {
	let msg = Message::from("Hello, World!");
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert_eq!(elements[0].as_text(), Some("Hello, World!"));
}

#[test]
fn test_message_from_empty_string() {
	let msg = Message::from("");
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert_eq!(elements[0].as_text(), Some(""));
}

#[test]
fn test_message_from_owned_string() {
	let msg = Message::from(String::from("Owned message"));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	match &elements[0] {
		Elements::Text(element) => {
			assert_eq!(element.text, "Owned message");
		}
		_ => panic!("Expected Text element"),
	}
}

#[test]
fn test_message_from_single_element() {
	let text = TextElement::new("Test message");
	let msg = Message::from(Elements::Text(text));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert_eq!(elements[0].as_text(), Some("Test message"));
}

#[test]
fn test_message_from_at_element() {
	let at = AtElement::new("123456");
	let msg = Message::from(Elements::At(at));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert!(elements[0].as_at().is_some());
	assert_eq!(elements[0].as_at().unwrap().target_id, "123456");
}

#[test]
fn test_message_from_face_element() {
	let face = FaceElement::new(42u64);
	let msg = Message::from(Elements::Face(face));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert!(elements[0].as_face().is_some());
	assert_eq!(elements[0].as_face().unwrap().id, 42u64);
}

#[test]
fn test_message_from_vec_empty() {
	let elements: Vec<Elements> = vec![];
	let msg = Message::from(elements.clone());
	let result: Vec<Elements> = msg.into();

	assert_eq!(result.len(), 0);
}

#[test]
fn test_message_from_vec_single() {
	let elements = vec![Elements::Text(TextElement::new("Single"))];
	let msg = Message::from(elements);
	let result: Vec<Elements> = msg.into();

	assert_eq!(result.len(), 1);
	assert_eq!(result[0].as_text(), Some("Single"));
}

#[test]
fn test_message_from_vec_multiple() {
	let elements = vec![
		Elements::At(AtElement::new("123456")),
		Elements::Text(TextElement::new(" Hello")),
		Elements::Face(FaceElement::new(178u64)),
	];
	let msg = Message::from(elements);
	let result: Vec<Elements> = msg.into();

	assert_eq!(result.len(), 3);
	assert!(result[0].as_at().is_some());
	assert_eq!(result[1].as_text(), Some(" Hello"));
	assert!(result[2].as_face().is_some());
}

#[test]
fn test_message_clone() {
	let msg1 = Message::from("Original");
	let msg2 = msg1.clone();

	let elements1: Vec<Elements> = msg1.into();
	let elements2: Vec<Elements> = msg2.into();

	assert_eq!(elements1.len(), elements2.len());
	assert_eq!(elements1[0].as_text(), elements2[0].as_text());
}

#[test]
fn test_message_debug() {
	let msg = Message::from("Debug test");
	let debug_str = format!("{:?}", msg);

	assert!(debug_str.contains("Message"));
}
