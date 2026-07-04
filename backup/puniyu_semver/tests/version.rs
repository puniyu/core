use puniyu_semver::Version;
use std::str::FromStr;

#[test]
fn test_new_sets_fields() {
	let version = Version::new(1, 2, 3);
	assert_eq!(version.major, 1);
	assert_eq!(version.minor, 2);
	assert_eq!(version.patch, 3);
}

#[test]
fn test_display_formats_core_version() {
	let version = Version::new(10, 20, 30);
	assert_eq!(version.to_string(), "10.20.30");
}

#[test]
fn test_from_str_core_version() {
	let parsed = Version::from_str("0.7.8");
	match parsed {
		Ok(version) => assert_eq!(version, Version::new(0, 7, 8)),
		Err(err) => panic!("expected valid version, got error: {err}"),
	}
}

#[test]
fn test_from_str_with_metadata_discards_extra_parts() {
	let parsed = Version::from_str("1.2.3-beta.1+build.9");
	match parsed {
		Ok(version) => assert_eq!(version, Version::new(1, 2, 3)),
		Err(err) => panic!("expected valid version with metadata, got error: {err}"),
	}
}

#[test]
fn test_from_str_invalid_version_should_fail() {
	let parsed = Version::from_str("1.2");
	assert!(parsed.is_err());
}

#[test]
fn test_from_semver_keeps_only_core_fields() {
	let parsed = semver::Version::parse("3.4.5-alpha.2+meta");
	let semver_value = match parsed {
		Ok(value) => value,
		Err(err) => panic!("failed to parse semver input: {err}"),
	};

	let version = Version::from(semver_value);
	assert_eq!(version, Version::new(3, 4, 5));
}

#[test]
fn test_into_semver_roundtrip_core_fields() {
	let parsed = semver::Version::parse("1.2.3-rc.1+build.9");
	let semver_value = match parsed {
		Ok(value) => value,
		Err(err) => panic!("failed to parse semver input: {err}"),
	};

	let version = Version::from(semver_value);
	let back_to_semver: semver::Version = version.into();

	assert_eq!(back_to_semver.major, 1);
	assert_eq!(back_to_semver.minor, 2);
	assert_eq!(back_to_semver.patch, 3);
	assert!(back_to_semver.pre.is_empty());
	assert!(back_to_semver.build.is_empty());
}

#[test]
fn test_serde_json_roundtrip() {
	let version = Version::new(5, 6, 7);

	let serialized = serde_json::to_string(&version);
	let serialized = match serialized {
		Ok(value) => value,
		Err(err) => panic!("failed to serialize version: {err}"),
	};
	assert_eq!(serialized, r#"{"major":5,"minor":6,"patch":7}"#);

	let deserialized = serde_json::from_str::<Version>(&serialized);
	match deserialized {
		Ok(value) => assert_eq!(value, version),
		Err(err) => panic!("failed to deserialize version: {err}"),
	}
}

#[test]
fn test_serde_json_missing_field_should_fail() {
	let deserialized = serde_json::from_str::<Version>(r#"{"major":1,"minor":2}"#);
	assert!(deserialized.is_err());
}
