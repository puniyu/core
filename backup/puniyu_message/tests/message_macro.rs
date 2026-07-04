use puniyu_element::send::{AtElement, Elements, FaceElement, TextElement};
use puniyu_message::message;

#[test]
fn test_macro_single_element() {
	let msg = message!(Elements::Text(TextElement::new("Hello")));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 1);
	assert_eq!(elements[0].as_text(), Some("Hello"));
}

#[test]
fn test_macro_multiple_elements() {
	let msg = message!(
		Elements::At(AtElement::new("123456")),
		Elements::Text(TextElement::new(" Hello")),
		Elements::Face(FaceElement::new(178u64)),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 3);
	assert!(elements[0].as_at().is_some());
	assert_eq!(elements[1].as_text(), Some(" Hello"));
	assert!(elements[2].as_face().is_some());
}

#[test]
fn test_macro_trailing_comma() {
	let msg = message!(
		Elements::Text(TextElement::new("First")),
		Elements::Text(TextElement::new("Second")),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 2);
}

#[test]
fn test_macro_no_trailing_comma() {
	let msg = message!(
		Elements::Text(TextElement::new("First")),
		Elements::Text(TextElement::new("Second"))
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 2);
}

#[test]
fn test_macro_with_variables() {
	let text = TextElement::new("Variable text");
	let at = AtElement::new("user123");

	let msg = message!(Elements::Text(text), Elements::At(at));
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 2);
	assert_eq!(elements[0].as_text(), Some("Variable text"));
	assert_eq!(elements[1].as_at().unwrap().target_id, "user123");
}

#[test]
fn test_macro_complex_message() {
	let msg = message!(
		Elements::At(AtElement::new("all")),
		Elements::Text(TextElement::new(" Announcement: ")),
		Elements::Text(TextElement::new("Important message")),
		Elements::Face(FaceElement::new(1u64)),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 4);
	assert_eq!(elements[0].as_at().unwrap().target_id, "all");
	assert_eq!(elements[1].as_text(), Some(" Announcement: "));
	assert_eq!(elements[2].as_text(), Some("Important message"));
	assert_eq!(elements[3].as_face().unwrap().id, 1u64);
}

#[test]
fn test_macro_only_text() {
	let msg = message!(
		Elements::Text(TextElement::new("Line 1")),
		Elements::Text(TextElement::new("Line 2")),
		Elements::Text(TextElement::new("Line 3")),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 3);
	assert_eq!(elements[0].as_text(), Some("Line 1"));
	assert_eq!(elements[1].as_text(), Some("Line 2"));
	assert_eq!(elements[2].as_text(), Some("Line 3"));
}

#[test]
fn test_macro_only_at() {
	let msg = message!(
		Elements::At(AtElement::new("user1")),
		Elements::At(AtElement::new("user2")),
		Elements::At(AtElement::new("user3")),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 3);
	assert_eq!(elements[0].as_at().unwrap().target_id, "user1");
	assert_eq!(elements[1].as_at().unwrap().target_id, "user2");
	assert_eq!(elements[2].as_at().unwrap().target_id, "user3");
}

#[test]
fn test_macro_only_face() {
	let msg = message!(
		Elements::Face(FaceElement::new(1u64)),
		Elements::Face(FaceElement::new(2u64)),
		Elements::Face(FaceElement::new(3u64)),
	);
	let elements: Vec<Elements> = msg.into();

	assert_eq!(elements.len(), 3);
	assert_eq!(elements[0].as_face().unwrap().id, 1u64);
	assert_eq!(elements[1].as_face().unwrap().id, 2u64);
	assert_eq!(elements[2].as_face().unwrap().id, 3u64);
}
