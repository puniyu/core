use crate::BotOption;
use puniyu_common::read_config;
use puniyu_path::config_dir;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};
static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| config_dir().join("bot.toml"));

/// Bot 配置结构
///
/// 管理所有 Bot 实例的配置，支持全局配置和单独 Bot 配置。
///
/// # 配置文件示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [bot.bot_001]
/// cd = 5
/// mode = 1
/// alias = ["小助手", "bot"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotConfig {
	/// 全局 Bot 配置
	///
	/// 作为所有 Bot 的默认配置
	#[serde(default)]
	global: BotOption,

	/// 特定 Bot 配置映射
	///
	/// 键为 Bot ID，值为该 Bot 的特定配置
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	bot: HashMap<SmolStr, BotOption>,
}

impl BotConfig {
	/// 获取当前 Bot 配置。
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path()).and_then(|v| v.try_into().ok()).unwrap_or_else(
			|| read_config::<Self>(config_dir().as_path(), "bot").unwrap_or_default(),
		)
	}

	/// 获取全局 Bot 配置。
	pub fn global(&self) -> &BotOption {
		&self.global
	}

	/// 获取指定 Bot 的配置，并自动与全局配置合并。
	pub fn bot(&self, bot_id: &str) -> BotOption {
		self.bot
			.get(bot_id)
			.map(|specific| crate::common::MergeWith::merge_with(specific, &self.global))
			.unwrap_or_else(|| self.global.clone())
	}

	/// 获取Bot列表
	pub fn list(&self) -> HashMap<&str, BotOption> {
		self.bot
			.iter()
			.map(|(id, specific)| {
				(id.as_str(), crate::common::MergeWith::merge_with(specific, &self.global))
			})
			.collect()
	}
}

impl crate::Config for BotConfig {
	fn name(&self) -> &str {
		"bot"
	}

	#[inline]
	fn to_value(&self) -> toml::Value {
		crate::serialize_to_value(self)
	}
}
