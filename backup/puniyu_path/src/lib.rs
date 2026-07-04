//! # puniyu_path
//!
//! 路径管理库，统一提供应用、插件与适配器目录路径。
//!
//! ## 特性
//!
//! - 提供基础目录路径函数（`app_cwd_dir`、`config_dir`、`data_dir` 等）
//! - 提供 `plugin` / `adapter` 子模块目录路径
//! - 所有路径基于 `puniyu_common::app` 当前应用信息生成

pub mod adapter;
pub mod plugin;

use puniyu_common::app::app_name;
use std::path::PathBuf;

/// 获取工作目录。
pub fn app_cwd_dir() -> PathBuf {
	puniyu_common::app::app_cwd_dir().to_path_buf()
}

/// 获取应用目录，格式为 `{app_dir}/@{app_name}`。
pub fn app_dir() -> PathBuf {
	let name = format!("@{name}", name = app_name());
	app_cwd_dir().join(name)
}

/// 获取日志目录，格式为 `{app_dir}/logs`。
pub fn log_dir() -> PathBuf {
	app_dir().join("logs")
}

/// 获取配置目录，格式为 `{app_dir}/config`。
pub fn config_dir() -> PathBuf {
	app_dir().join("config")
}

/// 获取临时目录，格式为 `{app_dir}/temp`。
pub fn temp_dir() -> PathBuf {
	app_dir().join("temp")
}

/// 获取插件目录，格式为 `{app_cwd_dir}/plugins`。
pub fn plugin_dir() -> PathBuf {
	app_cwd_dir().join("plugins")
}

/// 获取适配器目录，格式为 `{app_cwd_dir}/adapters`。
pub fn adapter_dir() -> PathBuf {
	app_cwd_dir().join("adapters")
}

/// 获取数据目录，格式为 `{app_dir}/data`。
pub fn data_dir() -> PathBuf {
	app_dir().join("data")
}

/// 获取资源目录，格式为 `{app_dir}/resources`。
pub fn resource_dir() -> PathBuf {
	app_dir().join("resources")
}
