use puniyu_contact::{
	Contact, ContactType, FriendContact, SceneType, contact, contact_friend, contact_group,
};
use std::str::FromStr;

fn snapshot(contact: impl Contact) -> (SceneType, String, Option<String>) {
	(*contact.scene(), contact.peer().to_string(), contact.name().map(str::to_string))
}

#[test]
fn test_contact_macro_flows_through_contact_type() {
	let contacts = [
		contact!(Friend, peer: "123456", name: "Alice"),
		contact!(Group, peer: "789012", name: "Dev Team"),
		contact!(GroupTemp, peer: "246810", name: "Temp Team"),
		contact!(Guild, peer: "9527", name: "Guild Channel"),
	];

	assert_eq!(
		snapshot(&contacts[0]),
		(SceneType::Friend, "123456".to_string(), Some("Alice".to_string()))
	);
	assert_eq!(
		snapshot(&contacts[1]),
		(SceneType::Group, "789012".to_string(), Some("Dev Team".to_string()))
	);
	assert_eq!(
		snapshot(&contacts[2]),
		(SceneType::GroupTemp, "246810".to_string(), Some("Temp Team".to_string()))
	);
	assert_eq!(
		snapshot(&contacts[3]),
		(SceneType::Guild, "9527".to_string(), Some("Guild Channel".to_string()))
	);
}

#[test]
fn test_contact_type_serde_roundtrip_preserves_variant_and_fields() {
	let friend = contact!(Friend, peer: "123456", name: "Alice");
	let friend_json = serde_json::to_string(&friend).unwrap();
	assert_eq!(friend_json, r#"{"type":"friend","field0":{"peer":"123456","name":"Alice"}}"#);

	let friend_roundtrip: ContactType<'_> = serde_json::from_str(&friend_json).unwrap();
	assert!(friend_roundtrip.is_friend());
	assert_eq!(
		snapshot(&friend_roundtrip),
		(SceneType::Friend, "123456".to_string(), Some("Alice".to_string()))
	);

	let group = contact!(Group, peer: "789012", name: "Dev Team");
	let group_json = serde_json::to_string(&group).unwrap();
	let group_roundtrip: ContactType<'_> = serde_json::from_str(&group_json).unwrap();
	assert!(group_roundtrip.is_group());
	assert_eq!(
		snapshot(&group_roundtrip),
		(SceneType::Group, "789012".to_string(), Some("Dev Team".to_string()))
	);

	let group_temp = contact!(GroupTemp, peer: "246810", name: "Temp Team");
	let group_temp_json = serde_json::to_string(&group_temp).unwrap();
	assert_eq!(
		group_temp_json,
		r#"{"type":"grouptemp","field0":{"peer":"246810","name":"Temp Team"}}"#
	);
	let group_temp_roundtrip: ContactType<'_> = serde_json::from_str(&group_temp_json).unwrap();
	assert!(group_temp_roundtrip.is_group_temp());
	assert!(!group_temp_roundtrip.is_group());
	assert_eq!(
		snapshot(&group_temp_roundtrip),
		(SceneType::GroupTemp, "246810".to_string(), Some("Temp Team".to_string()))
	);
}

#[test]
fn test_contact_type_deserializes_borrowed_fields_from_json() {
	let json = r#"{"type":"friend","field0":{"peer":"123456","name":"Alice"}}"#;
	let contact: ContactType<'_> = serde_json::from_str(json).unwrap();

	match contact {
		ContactType::Friend(friend) => {
			assert_eq!(friend.peer(), "123456");
			assert_eq!(friend.name(), Some("Alice"));
		}
		ContactType::Group(_) | ContactType::GroupTemp(_) | ContactType::Guild(_) => {
			panic!("expected friend contact")
		}
	}
}

#[test]
fn test_scene_type_supports_string_and_json_roundtrip() {
	assert_eq!(SceneType::Friend.to_string(), "friend");
	assert_eq!(SceneType::from_str("group").unwrap(), SceneType::Group);
	assert_eq!(SceneType::from_str("grouptemp").unwrap(), SceneType::GroupTemp);
	assert_eq!(SceneType::from_str("guild").unwrap(), SceneType::Guild);

	let json = serde_json::to_string(&SceneType::Friend).unwrap();
	assert_eq!(json, r#""friend""#);
	assert_eq!(serde_json::to_string(&SceneType::GroupTemp).unwrap(), r#""grouptemp""#);
	assert_eq!(serde_json::to_string(&SceneType::Guild).unwrap(), r#""guild""#);

	let decoded: SceneType = serde_json::from_str(&json).unwrap();
	assert_eq!(decoded, SceneType::Friend);
}

#[test]
fn test_trait_object_equality_and_reference_impl() {
	let friend = contact_friend!(peer: "123456", name: "Alice");
	let same_friend = ContactType::new(SceneType::Friend, "123456", Some("Alice"));
	let group = contact_group!(peer: "123456", name: "Alice");

	let left: &dyn Contact = &friend;
	let right: &dyn Contact = &same_friend;
	let other: &dyn Contact = &group;

	assert!(left == right);
	assert!(left != other);
	assert_eq!(
		snapshot(&friend),
		(SceneType::Friend, "123456".to_string(), Some("Alice".to_string()))
	);
}

#[test]
fn test_friend_contact_roundtrip_through_public_api_layers() {
	let contact = contact!(Friend, peer: String::from("friend-001"), name: String::from("Alice"));
	let json = serde_json::to_string(&contact).unwrap();
	let decoded: ContactType<'_> = serde_json::from_str(&json).unwrap();

	assert_eq!(
		snapshot(&decoded),
		(SceneType::Friend, "friend-001".to_string(), Some("Alice".to_string()))
	);

	let borrowed_view = decoded.as_friend().unwrap();
	assert_eq!(borrowed_view.peer(), "friend-001");
	assert_eq!(borrowed_view.name(), Some("Alice"));

	let expected = FriendContact::new("friend-001", Some("Alice"));
	assert_eq!(contact, ContactType::Friend(expected));
}
