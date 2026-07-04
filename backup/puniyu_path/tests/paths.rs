use puniyu_path::*;
use std::path::{Path, PathBuf};

// 初始化测试环境
fn init_test_env() {
	use puniyu_common::app::{AppInfo, set_app_info};
	use std::sync::Once;

	static INIT: Once = Once::new();
	static VERSION: &puniyu_semver::Version = &puniyu_semver::Version::new(0, 1, 0);

	INIT.call_once(|| {
		let info = AppInfo::new("testapp", VERSION, Path::new("."));
		set_app_info(info);
	});
}

#[test]
fn test_working_dir() {
	init_test_env();
	use puniyu_common::app::app_cwd_dir;

	let working_dir = app_cwd_dir();
	assert_eq!(working_dir, &PathBuf::from("."));
}

#[test]
fn test_app_cwd_dir() {
	init_test_env();

	let dir = app_cwd_dir();
	assert_eq!(dir, PathBuf::from("."));
}

#[test]
fn test_app_dir() {
	init_test_env();

	let dir = app_dir();
	let expected = PathBuf::from(".").join("@testapp");
	assert_eq!(dir, expected);
}

#[test]
fn test_log_dir() {
	init_test_env();

	let dir = log_dir();
	let expected = app_dir().join("logs");
	assert_eq!(dir, expected);
}

#[test]
fn test_config_dir() {
	init_test_env();

	let dir = config_dir();
	let expected = app_dir().join("config");
	assert_eq!(dir, expected);
}

#[test]
fn test_temp_dir() {
	init_test_env();

	let dir = temp_dir();
	let expected = app_dir().join("temp");
	assert_eq!(dir, expected);
}

#[test]
fn test_plugin_dir() {
	init_test_env();

	let dir = plugin_dir();
	let expected = app_cwd_dir().join("plugins");
	assert_eq!(dir, expected);
}

#[test]
fn test_adapter_dir() {
	init_test_env();

	let dir = adapter_dir();
	let expected = app_cwd_dir().join("adapters");
	assert_eq!(dir, expected);
}

#[test]
fn test_data_dir() {
	init_test_env();

	let dir = data_dir();
	let expected = app_dir().join("data");
	assert_eq!(dir, expected);
}

#[test]
fn test_resource_dir() {
	init_test_env();

	let dir = resource_dir();
	let expected = app_dir().join("resources");
	assert_eq!(dir, expected);
}

#[test]
fn test_plugin_config_dir() {
	init_test_env();

	let dir = plugin::config_dir();
	let expected = app_dir().join("config").join("plugins");
	assert_eq!(dir, expected);
}

#[test]
fn test_plugin_data_dir() {
	init_test_env();

	let dir = plugin::data_dir();
	let expected = app_dir().join("data").join("plugins");
	assert_eq!(dir, expected);
}

#[test]
fn test_plugin_resource_dir() {
	init_test_env();

	let dir = plugin::resource_dir();
	let expected = app_dir().join("resources").join("plugins");
	assert_eq!(dir, expected);
}

#[test]
fn test_plugin_temp_dir() {
	init_test_env();

	let dir = plugin::temp_dir();
	let expected = app_dir().join("temp").join("plugins");
	assert_eq!(dir, expected);
}

#[test]
fn test_adapter_config_dir() {
	init_test_env();

	let dir = adapter::config_dir();
	let expected = app_dir().join("config").join("adapters");
	assert_eq!(dir, expected);
}

#[test]
fn test_adapter_data_dir() {
	init_test_env();

	let dir = adapter::data_dir();
	let expected = app_dir().join("data").join("adapters");
	assert_eq!(dir, expected);
}

#[test]
fn test_adapter_resource_dir() {
	init_test_env();

	let dir = adapter::resource_dir();
	let expected = app_dir().join("resources").join("adapters");
	assert_eq!(dir, expected);
}

#[test]
fn test_adapter_temp_dir() {
	init_test_env();

	let dir = adapter::temp_dir();
	let expected = app_dir().join("temp").join("adapters");
	assert_eq!(dir, expected);
}

#[test]
fn test_path_exists() {
	init_test_env();

	let _ = app_cwd_dir();
	let _ = app_dir();
	let _ = log_dir();
	let _ = config_dir();
	let _ = temp_dir();
	let _ = plugin_dir();
	let _ = adapter_dir();
	let _ = data_dir();
	let _ = resource_dir();
}

#[test]
fn test_path_components() {
	init_test_env();

	let dir = app_dir();
	assert!(dir.ends_with("@testapp"));

	let dir = log_dir();
	assert!(dir.ends_with("logs"));

	let dir = config_dir();
	assert!(dir.ends_with("config"));

	let dir = plugin_dir();
	assert!(dir.ends_with("plugins"));

	let dir = adapter_dir();
	assert!(dir.ends_with("adapters"));
}
