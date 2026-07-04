#![cfg(feature = "config")]

use puniyu_error::config::Error;
use std::io;

#[test]
fn test_io_error_conversion() {
	let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
	let error: Error = io_err.into();

	match error {
		Error::Io(_) => (),
		_ => panic!("Expected Io error"),
	}
}

#[test]
fn test_io_error_display() {
	let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
	let error = Error::Io(io_err);
	let error_str = error.to_string();

	assert!(error_str.contains("IO error"));
}

#[test]
fn test_deserialize_error_conversion() {
	let toml_str = "invalid toml [[[";
	let result: Result<toml::Value, toml::de::Error> = toml::from_str(toml_str);

	if let Err(toml_err) = result {
		let error: Error = toml_err.into();
		match error {
			Error::Deserialize(_) => (),
			_ => panic!("Expected Deserialize error"),
		}
	}
}

#[test]
fn test_deserialize_error_display() {
	let toml_str = "invalid = [[[";
	let result: Result<toml::Value, toml::de::Error> = toml::from_str(toml_str);

	if let Err(toml_err) = result {
		let error = Error::Deserialize(toml_err);
		let error_str = error.to_string();
		assert!(error_str.contains("TOML parse error"));
	}
}

#[test]
fn test_error_debug() {
	let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
	let error = Error::Io(io_err);
	let debug_str = format!("{:?}", error);

	assert!(debug_str.contains("Io"));
}

#[test]
fn test_error_from_io() {
	fn read_file() -> Result<String, Error> {
		std::fs::read_to_string("nonexistent_file.txt")?;
		Ok(String::new())
	}

	let result = read_file();
	assert!(result.is_err());

	if let Err(error) = result {
		match error {
			Error::Io(_) => (),
			_ => panic!("Expected Io error"),
		}
	}
}

#[test]
fn test_error_chain() {
	fn load_config() -> Result<toml::Value, Error> {
		let content = std::fs::read_to_string("test.toml")?;
		let value = toml::from_str(&content)?;
		Ok(value)
	}

	let result = load_config();
	assert!(result.is_err());
}
