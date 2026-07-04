use bytes::Bytes;
use puniyu_account::AccountInfo;

#[test]
fn test_account_info_builder_full() {
	let account = AccountInfo::builder()
		.uin("123456789")
		.name("MyBot")
		.avatar(Bytes::from_static(b"image data"))
		.build();

	assert_eq!(
		account,
		AccountInfo {
			uin: "123456789".to_string(),
			name: "MyBot".to_string(),
			avatar: Bytes::from_static(b"image data"),
		}
	);
}

#[test]
fn test_account_info_builder_setter_into() {
	let account = AccountInfo::builder()
		.uin(String::from("123456789"))
		.name(String::from("MyBot"))
		.avatar(Vec::from(&b"avatar"[..]))
		.build();

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from_static(b"avatar"));
}
