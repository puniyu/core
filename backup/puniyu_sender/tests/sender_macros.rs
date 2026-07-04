use puniyu_sender::{Role, Sender, Sex, sender_friend, sender_group};

#[test]
fn test_friend_macro_full() {
	let sender = sender_friend!(
		user_id: "123456",
		nick: "Alice",
		sex: Sex::Female,
		age: 25u32,
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_friend_macro_minimal() {
	let sender = sender_friend!("123456");
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_friend_macro_owned_strings() {
	let sender = sender_friend!(user_id: String::from("123456"), nick: String::from("Alice"));

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
}

#[test]
fn test_group_macro_full() {
	let sender = sender_group!(
		user_id: "123456",
		nick: "Alice",
		sex: Sex::Female,
		age: 25u32,
		role: Role::Admin,
		card: "管理员",
		level: 10u32,
		title: "活跃成员",
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}

#[test]
fn test_group_macro_minimal() {
	let sender = sender_group!("123456");
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_group_macro_owned_strings() {
	let sender = sender_group!(
		user_id: String::from("123456"),
		nick: String::from("Alice"),
		card: String::from("管理员"),
		title: String::from("活跃成员"),
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}

#[test]
fn test_group_macro_roles() {
	let member = sender_group!(user_id: "111", role: Role::Member);
	let admin = sender_group!(user_id: "222", role: Role::Admin);
	let owner = sender_group!(user_id: "333", role: Role::Owner);

	assert_eq!(member.role(), &Role::Member);
	assert_eq!(admin.role(), &Role::Admin);
	assert_eq!(owner.role(), &Role::Owner);
}
