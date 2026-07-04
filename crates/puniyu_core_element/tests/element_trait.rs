use puniyu_core_element::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TestElementType {
	Text,
	Image,
	At,
	Reply,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct TestElement {
	kind: TestElementType,
}

impl Element for TestElement {
	type ElementType = TestElementType;
	fn r#type(&self) -> Self::ElementType {
		self.kind
	}
}

fn text() -> TestElement {
	TestElement { kind: TestElementType::Text }
}

fn image() -> TestElement {
	TestElement { kind: TestElementType::Image }
}

#[test]
fn trait_method_returns() {
	assert_eq!(text().r#type(), TestElementType::Text);
	assert_eq!(image().r#type(), TestElementType::Image);
}

#[test]
fn blanket_impl_for_ref() {
	let e = text();
	let r: &TestElement = &e;
	let r2: &&TestElement = &r;
	assert_eq!(r.r#type(), TestElementType::Text);
	assert_eq!(r2.r#type(), TestElementType::Text);
}

#[test]
fn concrete_partial_eq() {
	assert_eq!(text(), text());
	assert_ne!(text(), image());
}

#[test]
fn element_type_is_copy_and_independent() {
	let e = text();
	let copied: TestElementType = e.r#type();
	let _still_owned = e;
	assert_eq!(copied, TestElementType::Text);
}

#[test]
fn element_type_serialize_roundtrip() {
	let kind = TestElementType::Text;
	let json = serde_json::to_string(&kind).unwrap();
	assert_eq!(json, "\"text\"");
	let decoded: TestElementType = serde_json::from_str(&json).unwrap();
	assert_eq!(decoded, kind);
}

#[test]
fn element_type_serialize_all_variants() {
	assert_eq!(serde_json::to_string(&TestElementType::Text).unwrap(), "\"text\"");
	assert_eq!(serde_json::to_string(&TestElementType::Image).unwrap(), "\"image\"");
	assert_eq!(serde_json::to_string(&TestElementType::At).unwrap(), "\"at\"");
	assert_eq!(serde_json::to_string(&TestElementType::Reply).unwrap(), "\"reply\"");
}

#[test]
fn element_type_deserialize_from_str() {
	let text: TestElementType = serde_json::from_str("\"text\"").unwrap();
	assert_eq!(text, TestElementType::Text);
	let reply: TestElementType = serde_json::from_str("\"reply\"").unwrap();
	assert_eq!(reply, TestElementType::Reply);
}