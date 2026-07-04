use puniyu_contact::{Contact, FriendContact, contact_friend};

#[test]
fn test_creation() {
	let friend = FriendContact::new("123456", Some("Alice"));

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_trait_methods() {
	let friend = FriendContact::new("123456", Some("Alice"));

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_clone_and_equality() {
	let friend1 = FriendContact::new("123456", Some("Alice"));
	let friend2 = friend1.clone();

	assert_eq!(friend1, friend2);
}

#[test]
fn test_macro() {
	let friend = contact_friend!("123456", "Alice");
	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}

#[test]
fn test_macro_with_owned_strings() {
	let friend = contact_friend!(String::from("123456"), String::from("Alice"));

	assert_eq!(friend.peer(), "123456");
	assert_eq!(friend.name(), Some("Alice"));
}
