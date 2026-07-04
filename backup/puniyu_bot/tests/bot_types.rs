use puniyu_bot::BotId;

#[test]
fn test_bot_id_from_u64() {
	let id: BotId = 123u64.into();
	assert!(matches!(id, BotId::Index(123)));
}

#[test]
fn test_bot_id_from_str() {
	let id: BotId = "123456".into();
	assert_eq!(id, BotId::SelfId("123456".into()));
}

#[test]
fn test_bot_id_equality() {
	let id1: BotId = 123u64.into();
	let id2: BotId = 123u64.into();
	assert_eq!(id1, id2);

	let id3: BotId = "123456".into();
	let id4: BotId = "123456".into();
	assert_eq!(id3, id4);
}

#[test]
fn test_bot_id_inequality() {
	let id1: BotId = 123u64.into();
	let id2: BotId = "123456".into();
	assert_ne!(id1, id2);
}

#[test]
fn test_bot_id_clone() {
	let id1: BotId = 123u64.into();
	let id2 = id1.clone();
	assert_eq!(id1, id2);
}

#[test]
fn test_bot_id_debug() {
	let id: BotId = 123u64.into();
	let debug_str = format!("{:?}", id);
	assert!(debug_str.contains("Index"));
	assert!(debug_str.contains("123"));
}
