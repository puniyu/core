use crate::FriendOption;
use puniyu_common::read_config;
use puniyu_path::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| config_dir().join("friend.toml"));

/// 好友配置结构
///
/// 管理所有好友的配置，支持全局配置和单独好友配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [friend.user_123]
/// cd = 5
/// mode = 1
/// alias = ["bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FriendConfig {
	/// 全局好友配置
	///
	/// 作为所有好友的默认配置
	#[serde(default)]
	global: FriendOption,

	/// 特定好友配置映射
	///
	/// 键为用户 ID，值为该好友的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	friend: HashMap<String, FriendOption>,
}

impl FriendConfig {
	/// 获取当前好友配置。
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path()).and_then(|v| v.try_into().ok()).unwrap_or_else(
			|| read_config::<Self>(config_dir().as_path(), "friend").unwrap_or_default(),
		)
	}

	/// 获取全局好友配置。
	pub fn global(&self) -> &FriendOption {
		&self.global
	}

	/// 获取指定好友的配置，并自动与全局配置合并。
	pub fn friend(&self, user_id: &str) -> FriendOption {
		self.friend
			.get(user_id)
			.map(|specific| crate::common::MergeWith::merge_with(specific, &self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取所有按用户 ID 定义的配置，并自动与全局配置合并。
	pub fn list(&self) -> Vec<FriendOption> {
		self.friend
			.values()
			.map(|specific| crate::common::MergeWith::merge_with(specific, &self.global))
			.collect()
	}
}

impl crate::Config for FriendConfig {
	fn name(&self) -> &str {
		"friend"
	}

	fn to_value(&self) -> toml::Value {
		crate::serialize_to_value(self)
	}
}
