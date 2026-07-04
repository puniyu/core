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

mod core;
#[doc(inline)]
pub use core::AppConfig;
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
mod logger;
mod registry;

pub use registry::ConfigRegistry;

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

pub(crate) fn serialize_to_value<T: serde::Serialize>(config: &T) -> toml::Value {
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

pub fn init() {
	macro_rules! register_config {
		($ty:ty) => {{
			let _ = ConfigRegistry::register(<$ty>::default());
		}};
	}

	register_config!(AppConfig);
	register_config!(BotConfig);
	register_config!(GroupConfig);
	register_config!(FriendConfig);

	config::start_config_watcher();
}
