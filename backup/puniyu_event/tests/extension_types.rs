use puniyu_event::extension::{NoticeSubEventType, RequestSubEventType};

#[test]
fn notice_sub_event_type_is_lightweight_and_stable() {
	let kind = NoticeSubEventType::new("friend_poke");
	let copied = kind.clone();

	assert_eq!(kind, copied);
	assert_eq!(kind.kind(), "friend_poke");
	assert_eq!(kind.to_string(), "friend_poke");
}

#[test]
fn request_sub_event_type_is_lightweight_and_stable() {
	let kind = RequestSubEventType::new("friend_add");
	let copied = kind.clone();

	assert_eq!(kind, copied);
	assert_eq!(kind.kind(), "friend_add");
	assert_eq!(kind.to_string(), "friend_add");
}
