use crate::types::GroupOption;
use puniyu_common::read_config;
use puniyu_path::config_dir;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| config_dir().join("group.toml"));

/// 群组配置结构
///
/// 管理所有群组的配置，支持全局配置和单独群组配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// user_cd = 0
/// mode = 0
/// alias = []
///
/// [group.group_123]
/// cd = 10
/// user_cd = 5
/// mode = 1
/// alias = ["bot"]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupConfig {
	/// 全局群组配置
	///
	/// 作为所有群组的默认配置
	#[serde(default)]
	global: GroupOption,

	/// 特定群组配置映射
	///
	/// 键为群组 ID，值为该群组的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	group: HashMap<String, GroupOption>,
}

impl GroupConfig {
	/// 获取当前群组配置。
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path()).and_then(|v| v.try_into().ok()).unwrap_or_else(
			|| read_config::<Self>(config_dir().as_path(), "group").unwrap_or_default(),
		)
	}

	/// 获取全局群组配置。
	pub fn global(&self) -> &GroupOption {
		&self.global
	}

	/// 获取指定群组的配置，并自动与全局配置合并。
	pub fn group(&self, group_id: &str) -> GroupOption {
		self.group
			.get(group_id)
			.map(|specific| crate::common::MergeWith::merge_with(specific, &self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取所有按群号定义的配置，并自动与全局配置合并。
	pub fn list(&self) -> Vec<GroupOption> {
		self.group
			.values()
			.map(|specific| crate::common::MergeWith::merge_with(specific, &self.global))
			.collect()
	}
}

impl crate::Config for GroupConfig {
	fn name(&self) -> &str {
		"group"
	}

	fn to_value(&self) -> toml::Value {
		crate::serialize_to_value(self)
	}
}
