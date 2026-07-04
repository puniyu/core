use bytes::Bytes;
use puniyu_account::AccountInfo;

#[test]
fn test_account_info_creation() {
	let account = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_default() {
	let account = AccountInfo::default();

	assert_eq!(account.uin, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_clone() {
	let account1 = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};
	let account2 = account1.clone();

	assert_eq!(account1, account2);
}

#[test]
fn test_account_info_equality() {
	let account1 = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};
	let account2 = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};
	let account3 = AccountInfo {
		uin: "987654321".to_string(),
		name: "OtherBot".to_string(),
		avatar: Bytes::from("other data"),
	};

	assert_eq!(account1, account2);
	assert_ne!(account1, account3);
}

#[test]
fn test_account_info_debug() {
	let account = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};
	let debug_str = format!("{:?}", account);

	assert!(debug_str.contains("AccountInfo"));
	assert!(debug_str.contains("123456789"));
	assert!(debug_str.contains("MyBot"));
}

#[test]
fn test_account_info_empty_fields() {
	let account = AccountInfo { uin: "".to_string(), name: "".to_string(), avatar: Bytes::new() };

	assert_eq!(account.uin, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_unicode_name() {
	let account = AccountInfo {
		uin: "123456789".to_string(),
		name: "我的机器人".to_string(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_special_characters() {
	let account = AccountInfo {
		uin: "user@123".to_string(),
		name: "Bot #1".to_string(),
		avatar: Bytes::from("binary\x00data"),
	};

	assert_eq!(account.uin, "user@123");
	assert_eq!(account.name, "Bot #1");
	assert!(account.avatar.contains(&b'\x00'));
}

#[test]
fn test_account_info_long_uin() {
	let account = AccountInfo {
		uin: "12345678901234567890".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.uin.len(), 20);
}

#[test]
fn test_account_info_serialization() {
	let account = AccountInfo {
		uin: "123456789".to_string(),
		name: "MyBot".to_string(),
		avatar: Bytes::from("image data"),
	};

	let json = serde_json::to_string(&account).unwrap();
	let deserialized: AccountInfo = serde_json::from_str(&json).unwrap();

	assert_eq!(account, deserialized);
}
