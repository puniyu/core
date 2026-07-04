use puniyu_event::message::MessageSubEventType;

#[test]
fn test_message_sub_type_display() {
	assert_eq!(MessageSubEventType::Friend.to_string(), "friend");
	assert_eq!(MessageSubEventType::Group.to_string(), "group");
	assert_eq!(MessageSubEventType::GroupTemp.to_string(), "grouptemp");
	assert_eq!(MessageSubEventType::Guild.to_string(), "guild");
}

#[test]
fn test_message_sub_type_from_str() {
	use std::str::FromStr;

	assert_eq!(MessageSubEventType::from_str("friend").unwrap(), MessageSubEventType::Friend);
	assert_eq!(MessageSubEventType::from_str("group").unwrap(), MessageSubEventType::Group);
	assert_eq!(MessageSubEventType::from_str("guild").unwrap(), MessageSubEventType::Guild);
}

#[test]
fn test_message_sub_type_equality() {
	assert_eq!(MessageSubEventType::Friend, MessageSubEventType::Friend);
	assert_eq!(MessageSubEventType::Group, MessageSubEventType::Group);
	assert_ne!(MessageSubEventType::Friend, MessageSubEventType::Group);
}

#[test]
fn test_message_sub_type_ordering() {
	assert!(MessageSubEventType::Friend < MessageSubEventType::Group);
	assert!(MessageSubEventType::Group < MessageSubEventType::Guild);
}

#[test]
fn test_message_sub_type_serialization() {
	use serde_json;

	let friend = MessageSubEventType::Friend;
	let json = serde_json::to_string(&friend).unwrap();
	assert_eq!(json, r#""friend""#);

	let group = MessageSubEventType::Group;
	let json = serde_json::to_string(&group).unwrap();
	assert_eq!(json, r#""group""#);
}

#[test]
fn test_message_sub_type_deserialization() {
	use serde_json;

	let json = r#""friend""#;
	let sub_type: MessageSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(sub_type, MessageSubEventType::Friend);

	let json = r#""group""#;
	let sub_type: MessageSubEventType = serde_json::from_str(json).unwrap();
	assert_eq!(sub_type, MessageSubEventType::Group);
}
