use puniyu_sender::{
	FriendSender, GroupSender, GroupTempSender, GuildSender, Role, Sender, SenderType, Sex,
};

#[test]
fn test_from_friend() {
	let friend = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));
	let sender = SenderType::from(friend);

	assert!(sender.is_friend());
	assert!(!sender.is_group());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_from_group() {
	let group = GroupSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("管理员"),
		Some(10),
		Some("活跃成员"),
	);
	let sender = SenderType::from(group);

	assert!(sender.is_group());
	assert!(!sender.is_friend());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_as_friend() {
	let friend = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));
	let sender = SenderType::Friend(friend);

	assert!(sender.as_friend().is_some());
	assert!(sender.as_group().is_none());
	assert_eq!(sender.as_friend().unwrap().user_id(), "123456");
}

#[test]
fn test_as_group() {
	let group = GroupSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("管理员"),
		Some(10),
		Some("活跃成员"),
	);
	let sender = SenderType::Group(group);

	assert!(sender.as_group().is_some());
	assert!(sender.as_friend().is_none());

	let g = sender.as_group().unwrap();
	assert_eq!(g.user_id(), "123456");
	assert_eq!(g.role(), &Role::Admin);
	assert_eq!(g.card(), Some("管理员"));
}

#[test]
fn test_trait_methods() {
	let friend = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));
	let sender = SenderType::Friend(friend);

	assert_eq!(sender.user_id(), "123456");
	assert_eq!(sender.name(), Some("Alice"));
	assert_eq!(sender.sex(), &Sex::Female);
	assert_eq!(sender.age(), Some(25));
}

#[test]
fn test_group_temp_sender() {
	let group_temp = GroupTempSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("临时管理员"),
		Some(10),
		Some("临时成员"),
	);
	let sender = SenderType::GroupTemp(group_temp);

	assert!(sender.is_group_temp());
	assert!(!sender.is_group());
	assert!(sender.as_group_temp().is_some());
	assert!(sender.as_group().is_none());
	assert_eq!(sender.user_id(), "123456");
}

#[test]
fn test_guild_sender() {
	let guild = GuildSender::new(
		"123456",
		Some("Alice"),
		Sex::Female,
		Some(25),
		Role::Admin,
		Some("频道管理员"),
		Some(10),
		Some("频道成员"),
	);
	let sender = SenderType::Guild(guild);

	assert!(sender.is_guild());
	assert!(!sender.is_group());
	assert!(sender.as_guild().is_some());
	assert!(sender.as_group().is_none());
	assert_eq!(sender.user_id(), "123456");
}
