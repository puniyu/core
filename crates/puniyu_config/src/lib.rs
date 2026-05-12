//! # puniyu_config
//!
//! 统一的 puniyu 配置管理库，覆盖应用、Bot、群聊与好友场景。
//!
//! ## 特性
//!
//! - 提供 `AppConfig`、`BotConfig`、`GroupConfig`、`FriendConfig`
//! - 提供 `app_config()`、`bot_config()`、`group_config()`、`friend_config()` 统一访问入口
//! - 提供 `ConfigRegistry` 管理已注册配置
//! - 初始化时自动创建配置目录并启动配置监听
//!
//! ## 示例
//!
//! ```rust,no_run
//! use puniyu_config::{app_config, bot_config, init};
//!
//! init();
//!
//! let app = app_config();
//! let bot = bot_config().bot("bot_001");
//!
//! assert_eq!(app.prefix().as_deref(), Some("!"));
//! let _ = bot.cd();
//! ```

mod app;
#[doc(inline)]
pub use app::AppConfig;
mod bot;
#[doc(inline)]
pub use bot::BotConfig;
mod friend;
#[doc(inline)]
pub use friend::FriendConfig;
mod group;
#[doc(inline)]
pub use group::GroupConfig;
mod types;
#[doc(inline)]
pub use types::*;
mod common;
mod config;
mod registry;

pub use registry::ConfigRegistry;

use log::{debug, error, info};
use puniyu_common::merge_config;
use puniyu_path::{config_dir, log_dir};

/// 配置 trait
pub trait Config: Send + Sync {
	/// 配置名称（对应文件名，不含扩展名）
	fn name(&self) -> &str;
	/// 配置文件所在目录
	fn path(&self) -> std::path::PathBuf {
		puniyu_path::config_dir()
	}
	/// 序列化为 toml::Value
	fn to_value(&self) -> toml::Value;
}

impl PartialEq for dyn Config {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.path() == other.path()
	}
}

pub(crate)  fn serialize_to_value<T: serde::Serialize>(config: &T) -> toml::Value {
	toml::Value::try_from(config).expect("Failed to serialize config to toml::Value")
}

/// 获取应用配置。
#[inline]
pub fn app_config() -> AppConfig {
	AppConfig::get()
}

/// 获取 Bot 配置。
#[inline]
pub fn bot_config() -> BotConfig {
	BotConfig::get()
}

/// 获取好友配置。
#[inline]
pub fn friend_config() -> FriendConfig {
	FriendConfig::get()
}

/// 获取群组配置。
#[inline]
pub fn group_config() -> GroupConfig {
	GroupConfig::get()
}

/// 初始化配置目录、合并默认配置并启动配置监听。
pub fn init() {
	if !config_dir().as_path().exists() {
		std::fs::create_dir_all(config_dir().as_path())
			.unwrap_or_else(|_| error!("[Config] Failed to initialize config directory"));
	}
	if !log_dir().as_path().exists() {
		std::fs::create_dir_all(log_dir().as_path())
			.unwrap_or_else(|_| error!("[Config] Failed to initialize log directory"));
	}

	merge_config(config_dir().as_path(), "app", &AppConfig::default(), &AppConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge app config: {}", e);
		});
	merge_config(config_dir().as_path(), "group", &GroupConfig::default(), &GroupConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge group config: {}", e);
		});
	merge_config(config_dir().as_path(), "friend", &FriendConfig::default(), &FriendConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge friend config: {}", e);
		});
	merge_config(config_dir().as_path(), "bot", &BotConfig::default(), &BotConfig::get())
		.unwrap_or_else(|e| {
			error!("[Config] Failed to merge bot config: {}", e);
		});

	let app_config = AppConfig::get();
	if let Err(e) = ConfigRegistry::register(app_config) {
		error!("[Config] Failed to register app config: {}", e);
	} else {
		info!("[Config] App config registered");
	}

	let bot_config = BotConfig::get();
	if let Err(e) = ConfigRegistry::register(bot_config) {
		error!("[Config] Failed to register bot config: {}", e);
	} else {
		debug!("[Config] Bot config registered");
	}

	let group_config = GroupConfig::get();
	if let Err(e) = ConfigRegistry::register(group_config) {
		error!("[Config] Failed to register group config: {}", e);
	} else {
		debug!("[Config] Group config registered");
	}

	let friend_config = FriendConfig::get();
	if let Err(e) = ConfigRegistry::register(friend_config) {
		error!("[Config] Failed to register friend config: {}", e);
	} else {
		debug!("[Config] Friend config registered");
	}
	config::start_config_watcher();
}
