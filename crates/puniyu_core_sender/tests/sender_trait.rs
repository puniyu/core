use puniyu_core_sender::Sender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TestSex {
	Male,
	Female,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct TestSender {
	user_id: String,
	name: Option<String>,
	sex: TestSex,
	age: Option<u32>,
}

impl Sender for TestSender {
	type Sex = TestSex;

	fn user_id(&self) -> &str {
		&self.user_id
	}
	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
	fn sex(&self) -> Self::Sex {
		self.sex
	}
	fn age(&self) -> Option<u32> {
		self.age
	}
}

fn alice() -> TestSender {
	TestSender {
		user_id: "u1".into(),
		name: Some("Alice".into()),
		sex: TestSex::Female,
		age: Some(25),
	}
}

fn bob() -> TestSender {
	TestSender {
		user_id: "u2".into(),
		name: Some("Bob".into()),
		sex: TestSex::Male,
		age: Some(30),
	}
}

#[test]
fn trait_method_returns() {
	let s = alice();
	assert_eq!(s.user_id(), "u1");
	assert_eq!(s.name(), Some("Alice"));
	assert_eq!(s.sex(), TestSex::Female);
	assert_eq!(s.age(), Some(25));
}

#[test]
fn trait_method_returns_none_name() {
	let s = TestSender {
		user_id: "u3".into(),
		name: None,
		sex: TestSex::Unknown,
		age: None,
	};
	assert_eq!(s.name(), None);
	assert_eq!(s.age(), None);
}

#[test]
fn blanket_impl_for_ref() {
	let s = alice();
	let r: &TestSender = &s;
	let r2: &&TestSender = &r;
	assert_eq!(r.user_id(), "u1");
	assert_eq!(r.sex(), TestSex::Female);
	assert_eq!(r2.user_id(), "u1");
	assert_eq!(r2.sex(), TestSex::Female);
}

#[test]
fn concrete_partial_eq() {
	assert_eq!(alice(), alice());
	assert_ne!(alice(), bob());
}

#[test]
fn sex_is_copy_and_independent() {
	let s = alice();
	let copied: TestSex = s.sex();
	let _still_owned = s;
	assert_eq!(copied, TestSex::Female);
}

#[test]
fn sex_serialize_roundtrip() {
	let sex = TestSex::Female;
	let json = serde_json::to_string(&sex).unwrap();
	assert_eq!(json, "\"female\"");
	let decoded: TestSex = serde_json::from_str(&json).unwrap();
	assert_eq!(decoded, sex);
}

#[test]
fn sex_serialize_all_variants() {
	assert_eq!(serde_json::to_string(&TestSex::Male).unwrap(), "\"male\"");
	assert_eq!(serde_json::to_string(&TestSex::Female).unwrap(), "\"female\"");
	assert_eq!(serde_json::to_string(&TestSex::Unknown).unwrap(), "\"unknown\"");
}

#[test]
fn sex_deserialize_from_str() {
	let male: TestSex = serde_json::from_str("\"male\"").unwrap();
	assert_eq!(male, TestSex::Male);
	let unknown: TestSex = serde_json::from_str("\"unknown\"").unwrap();
	assert_eq!(unknown, TestSex::Unknown);
}