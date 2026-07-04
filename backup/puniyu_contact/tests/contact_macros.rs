use puniyu_contact::{Contact, contact_friend, contact_group};

#[test]
fn test_contact_friend_macro_peer_only() {
	let friend = contact_friend!("123456");

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), None);
}

#[test]
fn test_contact_friend_macro_with_name() {
	let friend = contact_friend!("123456", "Alice");

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_contact_friend_macro_named_fields() {
	let friend = contact_friend!(
		peer: "123456",
		name: "Alice",
	);

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_contact_friend_macro_peer_field_only() {
	let friend = contact_friend!(peer: "123456");

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), None);
}

#[test]
fn test_contact_friend_macro_empty_peer() {
	let friend = contact_friend!("");

	assert_eq!(friend.peer(), "");
	assert_eq!(friend.name(), None);
}

#[test]
fn test_contact_friend_macro_unicode() {
	let friend = contact_friend!("user123", "爱丽丝");

	assert_eq!(friend.peer(), "user123");
	assert_eq!(friend.name(), Some("爱丽丝"));
}

#[test]
fn test_contact_friend_macro_special_chars() {
	let friend = contact_friend!("user@123", "User #1");

	assert_eq!(friend.peer(), "user@123");
	assert_eq!(friend.name(), Some("User #1"));
}

// 测试 contact_group! 宏

#[test]
fn test_contact_group_macro_peer_only() {
	let group = contact_group!("789012");

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), None);
}

#[test]
fn test_contact_group_macro_with_name() {
	let group = contact_group!("789012", "Dev Team");

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_contact_group_macro_named_fields() {
	let group = contact_group!(
		peer: "789012",
		name: "Dev Team",
	);

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_contact_group_macro_peer_field_only() {
	let group = contact_group!(peer: "789012");

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), None);
}

#[test]
fn test_contact_group_macro_empty_peer() {
	let group = contact_group!("");

	assert_eq!(group.peer(), "");
	assert_eq!(group.name(), None);
}

#[test]
fn test_contact_group_macro_unicode() {
	let group = contact_group!("group123", "开发团队");

	assert_eq!(group.peer(), "group123");
	assert_eq!(group.name(), Some("开发团队"));
}

#[test]
fn test_contact_group_macro_special_chars() {
	let group = contact_group!("group@789", "Team #1");

	assert_eq!(group.peer(), "group@789");
	assert_eq!(group.name(), Some("Team #1"));
}

// 混合测试

#[test]
fn test_macros_in_vec() {
	let contacts = [contact_friend!("111", "User1"), contact_friend!("222")];

	assert_eq!(contacts.len(), 2);
	assert_eq!(contacts[0].peer(), "111");
	assert_eq!(contacts[1].peer(), "222");
}

#[test]
fn test_group_macros_in_vec() {
	let groups = [contact_group!("111", "Group1"), contact_group!("222")];

	assert_eq!(groups.len(), 2);
	assert_eq!(groups[0].peer(), "111");
	assert_eq!(groups[1].peer(), "222");
}
