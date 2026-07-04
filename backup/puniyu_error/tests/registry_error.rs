#![cfg(feature = "registry")]

use puniyu_error::registry::Error;

#[test]
fn test_not_found_error() {
	let error = Error::NotFound("test_item".to_string());
	assert_eq!(error.to_string(), "not found: test_item");
}

#[test]
fn test_exists_error() {
	let error = Error::Exists("test_item".to_string());
	assert_eq!(error.to_string(), "exists: test_item");
}

#[test]
fn test_not_found_with_empty_string() {
	let error = Error::NotFound(String::new());
	assert_eq!(error.to_string(), "not found: ");
}

#[test]
fn test_exists_with_empty_string() {
	let error = Error::Exists(String::new());
	assert_eq!(error.to_string(), "exists: ");
}

#[test]
fn test_not_found_with_unicode() {
	let error = Error::NotFound("测试项".to_string());
	assert_eq!(error.to_string(), "not found: 测试项");
}

#[test]
fn test_exists_with_unicode() {
	let error = Error::Exists("测试项".to_string());
	assert_eq!(error.to_string(), "exists: 测试项");
}

#[test]
fn test_not_found_with_special_chars() {
	let error = Error::NotFound("item@123".to_string());
	assert_eq!(error.to_string(), "not found: item@123");
}

#[test]
fn test_exists_with_special_chars() {
	let error = Error::Exists("item@123".to_string());
	assert_eq!(error.to_string(), "exists: item@123");
}

#[test]
fn test_error_debug() {
	let error = Error::NotFound("test".to_string());
	let debug_str = format!("{:?}", error);
	assert!(debug_str.contains("NotFound"));
	assert!(debug_str.contains("test"));
}

#[test]
fn test_error_match() {
	let error = Error::NotFound("item".to_string());

	match error {
		Error::NotFound(name) => assert_eq!(name, "item"),
		Error::Exists(_) => panic!("Expected NotFound"),
	}
}

#[test]
fn test_error_result() {
	fn get_item(name: &str) -> Result<(), Error> {
		Err(Error::NotFound(name.to_string()))
	}

	let result = get_item("test");
	assert!(result.is_err());

	if let Err(Error::NotFound(name)) = result {
		assert_eq!(name, "test");
	}
}

#[test]
fn test_error_propagation() {
	fn inner() -> Result<(), Error> {
		Err(Error::Exists("item".to_string()))
	}

	fn outer() -> Result<(), Error> {
		inner()?;
		Ok(())
	}

	let result = outer();
	assert!(result.is_err());
}

#[test]
fn test_not_found_clone() {
	let error1 = Error::NotFound("test".to_string());
	let error2 = Error::NotFound("test".to_string());

	assert_eq!(error1.to_string(), error2.to_string());
}

#[test]
fn test_exists_clone() {
	let error1 = Error::Exists("test".to_string());
	let error2 = Error::Exists("test".to_string());

	assert_eq!(error1.to_string(), error2.to_string());
}
