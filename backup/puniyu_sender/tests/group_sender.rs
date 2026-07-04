use puniyu_sender::{GroupSender, Role, Sender, Sex};

#[test]
fn test_creation() {
	let sender = GroupSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("管理员"),
		Some(10),
		Some("活跃成员"),
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.level(), Some(10));
	assert_eq!(sender.title(), Some("活跃成员"));
}

#[test]
fn test_trait_methods() {
	let sender = GroupSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("管理员"),
		Some(10),
		Some("活跃成员"),
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.role(), &Role::Admin);
	assert_eq!(sender.card(), Some("管理员"));
}

#[test]
fn test_different_roles() {
	let member = GroupSender::new(
		"111",
		Option::<&str>::None,
		Sex::Unknown,
		None,
		Role::Member,
		Option::<&str>::None,
		None,
		Option::<&str>::None,
	);
	let admin = GroupSender::new(
		"222",
		Option::<&str>::None,
		Sex::Unknown,
		None,
		Role::Admin,
		Option::<&str>::None,
		None,
		Option::<&str>::None,
	);
	let owner = GroupSender::new(
		"333",
		Option::<&str>::None,
		Sex::Unknown,
		None,
		Role::Owner,
		Option::<&str>::None,
		None,
		Option::<&str>::None,
	);

	assert_eq!(member.role(), &Role::Member);
	assert_eq!(admin.role(), &Role::Admin);
	assert_eq!(owner.role(), &Role::Owner);
}

#[test]
fn test_owned_string_creation() {
	let sender = GroupSender::new(
		String::from("123456"),
		Some(String::from("Alice")),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some(String::from("管理员")),
		Some(10),
		Some(String::from("活跃成员")),
	);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.card(), Some("管理员"));
	assert_eq!(sender.title(), Some("活跃成员"));
}
