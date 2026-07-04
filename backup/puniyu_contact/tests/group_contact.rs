use puniyu_contact::{Contact, GroupContact, contact_group};

#[test]
fn test_creation() {
	let group = GroupContact::new("789012", Some("Dev Team"));

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_trait_methods() {
	let group = GroupContact::new("789012", Some("Dev Team"));

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_clone_and_equality() {
	let group1 = GroupContact::new("789012", Some("Dev Team"));
	let group2 = group1.clone();

	assert_eq!(group1, group2);
}

#[test]
fn test_macro() {
	let group = contact_group!("789012", "Dev Team");
	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}

#[test]
fn test_macro_with_owned_strings() {
	let group = contact_group!(String::from("789012"), String::from("Dev Team"));

	assert_eq!(group.peer(), "789012");
	assert_eq!(group.name(), Some("Dev Team"));
}
