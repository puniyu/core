use puniyu_contact::{
	Contact, ContactType, FriendContact, GroupContact, GroupTempContact, GuildContact, SceneType,
};

#[test]
fn test_new_friend() {
	let contact = ContactType::new(SceneType::Friend, "123456", Some("Alice"));
	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_new_group() {
	let contact = ContactType::new(SceneType::Group, "789012", Some("Dev Team"));
	assert!(contact.is_group());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_new_group_temp() {
	let contact = ContactType::new(SceneType::GroupTemp, "246810", Some("Temp Team"));
	assert!(contact.is_group_temp());
	assert!(!contact.is_group());
	assert_eq!(contact.peer(), "246810");
	assert_eq!(contact.name(), Some("Temp Team"));
}

#[test]
fn test_new_guild() {
	let contact = ContactType::new(SceneType::Guild, "9527", Some("Guild Channel"));
	assert!(contact.is_guild());
	assert!(!contact.is_group());
	assert_eq!(contact.peer(), "9527");
	assert_eq!(contact.name(), Some("Guild Channel"));
}

#[test]
fn test_new_with_owned_strings() {
	let contact =
		ContactType::new(SceneType::Friend, String::from("123456"), Some(String::from("Alice")));

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_friend() {
	let friend = FriendContact::new("123456", Some("Alice"));
	let contact = ContactType::from(friend);

	assert!(contact.is_friend());
	assert!(contact.as_friend().is_some());
	assert_eq!(contact.peer(), "123456");
}

#[test]
fn test_from_group() {
	let group = GroupContact::new("789012", Some("Dev Team"));
	let contact = ContactType::from(group);

	assert!(contact.is_group());
	assert!(contact.as_group().is_some());
	assert_eq!(contact.peer(), "789012");
}

#[test]
fn test_as_friend() {
	let friend = FriendContact::new("123456", Some("Alice"));
	let contact = ContactType::Friend(friend);

	assert!(contact.as_friend().is_some());
	assert!(contact.as_group().is_none());
}

#[test]
fn test_as_group() {
	let group = GroupContact::new("789012", Some("Dev Team"));
	let contact = ContactType::Group(group);

	assert!(contact.as_group().is_some());
	assert!(contact.as_friend().is_none());
	assert!(contact.as_group_temp().is_none());
}

#[test]
fn test_as_group_temp() {
	let group = GroupTempContact::new("246810", Some("Temp Team"));
	let contact = ContactType::GroupTemp(group);

	assert!(contact.as_group_temp().is_some());
	assert!(contact.as_group().is_none());
	assert!(contact.as_friend().is_none());
}

#[test]
fn test_as_guild() {
	let guild = GuildContact::builder().peer("9527").name("Guild Channel").build();
	let contact = ContactType::Guild(guild);

	assert!(contact.as_guild().is_some());
	assert!(contact.as_group().is_none());
	assert!(contact.as_friend().is_none());
}

#[test]
fn test_trait_methods() {
	let friend = FriendContact::new("123456", Some("Alice"));
	let contact = ContactType::Friend(friend);

	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_friend_reference() {
	let friend = FriendContact::new("123456", Some("Alice"));
	let contact = ContactType::from(&friend);

	assert!(contact.is_friend());
	assert_eq!(contact.peer(), "123456");
	assert_eq!(contact.name(), Some("Alice"));
}

#[test]
fn test_from_group_reference() {
	let group = GroupContact::new("789012", Some("Dev Team"));
	let contact = ContactType::from(&group);

	assert!(contact.is_group());
	assert_eq!(contact.peer(), "789012");
	assert_eq!(contact.name(), Some("Dev Team"));
}

#[test]
fn test_from_group_temp_reference() {
	let group_temp = GroupTempContact::new("246810", Some("Temp Team"));
	let contact = ContactType::from(&group_temp);

	assert!(contact.is_group_temp());
	assert_eq!(contact.peer(), "246810");
	assert_eq!(contact.name(), Some("Temp Team"));
}

#[test]
fn test_from_guild_reference() {
	let guild = GuildContact::new("9527", Some("Guild Channel"), Some("General"));
	let contact = ContactType::from(&guild);

	assert!(contact.is_guild());
	assert_eq!(contact.peer(), "9527");
	assert_eq!(contact.name(), Some("Guild Channel"));
}
