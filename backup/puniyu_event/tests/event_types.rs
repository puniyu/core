use puniyu_event::EventType;

#[test]
fn test_event_type_display() {
	assert_eq!(EventType::Message.to_string(), "message");
	assert_eq!(EventType::Unknown.to_string(), "unknown");
}

#[test]
fn test_event_type_default() {
	let default_type = EventType::default();
	assert_eq!(default_type, EventType::Unknown);
}

#[test]
fn test_event_type_from_str() {
	use std::str::FromStr;

	assert_eq!(EventType::from_str("message").unwrap(), EventType::Message);
}
