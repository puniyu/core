use bytes::Bytes;
use puniyu_adapter_types::{AdapterPlatform, AdapterProtocol, Avatar, MessageType, adapter_info};
use puniyu_semver::Version;

#[test]
fn adapter_info_short_macro_builds_basic_info() {
	let info = adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console);

	assert_eq!(info.name, "console");
	assert_eq!(info.platform, AdapterPlatform::QQ);
	assert_eq!(info.protocol, AdapterProtocol::Console);
}

#[test]
fn adapter_info_named_macro_applies_custom_fields() {
	let info = adapter_info!(
		name: "napcat",
		author: vec!["Puniyu".into()],
		version: Version::new(1, 2, 3),
		address: "127.0.0.1:8080".to_string(),
		secret: "token".to_string(),
	);

	assert_eq!(info.name, "napcat");
	assert_eq!(info.author.first().map(|s| s.as_str()), Some("Puniyu"));
	assert_eq!(info.version, Version::new(1, 2, 3));
	assert_eq!(info.address.as_deref(), Some("127.0.0.1:8080"));
	assert_eq!(info.secret.as_deref(), Some("token"));
}

#[test]
fn message_type_from_converts_string_and_sequence() {
	assert!(matches!(MessageType::from("12345"), MessageType::Id(id) if id == "12345"));
	assert!(matches!(MessageType::from(String::from("abc")), MessageType::Id(id) if id == "abc"));
	assert!(matches!(MessageType::from(42_u64), MessageType::Seq(42)));
}

#[test]
fn adapter_platform_supports_display_and_parse() {
	assert_eq!(AdapterPlatform::QQ.to_string(), "qq");
	assert_eq!("telegram".parse::<AdapterPlatform>().unwrap(), AdapterPlatform::Telegram);
}

#[test]
fn avatar_exposes_original_bytes() {
	let bytes = Bytes::from_static(b"avatar");
	let avatar = Avatar::new(&bytes);

	assert_eq!(avatar.bytes(), &bytes);
}
