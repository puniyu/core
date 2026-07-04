use bytes::Bytes;
use puniyu_account::account_info;

#[test]
fn test_account_info_macro_full() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: Bytes::from("image data"),
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_macro_unicode() {
	let account = account_info!(
		uin: "123456789",
		name: "我的机器人",
		avatar: Bytes::from("image data"),
	);

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_macro_trailing_comma() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: Bytes::from("image data"),
	);

	assert_eq!(account.uin, "123456789");
}

#[test]
fn test_account_info_macro_no_trailing_comma() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: Bytes::from("image data")
	);

	assert_eq!(account.uin, "123456789");
}

#[test]
fn test_account_info_macro_string_conversion() {
	let uin = String::from("123456789");
	let name = String::from("MyBot");
	let avatar = Bytes::from("image data");

	let account = account_info!(
		uin: uin,
		name: name,
		avatar: avatar,
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_macro_str_slice() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: Bytes::from("image data"),
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_macro_empty_strings() {
	let account = account_info!(
		uin: "",
		name: "",
		avatar: Bytes::new(),
	);

	assert_eq!(account.uin, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_macro_special_characters() {
	let account = account_info!(
		uin: "user@123",
		name: "Bot #1",
		avatar: Bytes::from("binary\x00data"),
	);

	assert_eq!(account.uin, "user@123");
	assert_eq!(account.name, "Bot #1");
	assert!(account.avatar.contains(&b'\x00'));
}

#[test]
fn test_account_info_macro_long_values() {
	let account = account_info!(
		uin: "12345678901234567890",
		name: "MyVeryLongBotNameWithManyCharacters",
		avatar: Bytes::from(vec![0u8; 100]),
	);

	assert_eq!(account.uin.len(), 20);
	assert!(account.name.len() > 20);
	assert!(account.avatar.len() == 100);
}

#[test]
fn test_account_info_macro_positional() {
	let account = account_info!("123456789", "MyBot", Bytes::from("image data"));

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_macro_positional_empty() {
	let account = account_info!("123456789", "MyBot", Bytes::new());

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_macro_positional_unicode() {
	let account = account_info!("123456789", "我的机器人", Bytes::from("image data"));

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_macro_positional_string() {
	let uin = String::from("123456789");
	let name = String::from("MyBot");
	let avatar = Bytes::from("image data");

	let account = account_info!(uin, name, avatar);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_macro_named_fields_out_of_order() {
	let account = account_info!(
		avatar: Bytes::from_static(b"image data"),
		name: "MyBot",
		uin: "123456789",
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from_static(b"image data"));
}
