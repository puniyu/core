//! 子事件类型综合测试

use puniyu_event::message::MessageSubEventType;
use std::str::FromStr;

#[test]
fn test_all_message_sub_types() {
	let types = vec![
		(MessageSubEventType::Friend, "friend"),
		(MessageSubEventType::Group, "group"),
		(MessageSubEventType::GroupTemp, "grouptemp"),
		(MessageSubEventType::Guild, "guild"),
	];

	for (sub_type, expected_str) in types {
		assert_eq!(sub_type.to_string(), expected_str);
		assert_eq!(MessageSubEventType::from_str(expected_str).unwrap(), sub_type);
	}
}

#[test]
fn test_sub_type_debug() {
	let msg_type = MessageSubEventType::Friend;
	let debug_str = format!("{:?}", msg_type);
	assert!(!debug_str.is_empty());
}
