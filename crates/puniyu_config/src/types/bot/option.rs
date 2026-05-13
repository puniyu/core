use crate::{ReactiveMode, default_cd};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::types::empty_vec_as_none;

/// Bot 配置选项
///
/// 定义单个 Bot 实例的配置参数。
///
/// # 配置继承
///
/// 特定 Bot 配置会继承全局配置。如果某个字段未设置（`None`），
/// 则使用全局配置中的对应值。
///
/// # 配置项
///
/// - `cd`: 消息冷却时间（秒），防止频繁响应
/// - `mode`: 响应模式，控制 Bot 响应哪些消息
/// - `alias`: Bot 别名列表，用于命令识别
///
/// # 示例
///
/// ```toml
/// [global]
/// cd = 0
/// mode = 0
/// alias = []
///
/// [bot.bot_001]
/// # 只覆盖 cd 和 mode，alias 继承全局配置
/// cd = 5
/// mode = 1
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotOption {
	/// 消息冷却时间（秒）
	///
	/// 设置 Bot 响应消息的最小间隔时间，防止频繁响应
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	cd: Option<u64>,

	/// 响应模式
	///
	/// 控制 Bot 响应哪些类型的消息
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	mode: Option<ReactiveMode>,

	/// Bot 别名列表
	///
	/// 用于命令识别，当消息以别名开头时会被识别为命令
	/// 如果未设置，继承全局配置
	#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "empty_vec_as_none")]
	alias: Option<Vec<SmolStr>>,
}

impl Default for BotOption {
	#[inline]
	fn default() -> Self {
		Self {
			cd: Some(default_cd()),
			mode: Some(Default::default()),
			alias: None,
		}
	}
}

impl crate::common::MergeWith for BotOption {
	fn merge_with(&self, global: &Self) -> Self {
		Self {
			cd: self.cd.or(global.cd),
			mode: self.mode.or(global.mode),
			alias: self.alias.clone().or(global.alias.clone()),
		}
	}
}

impl BotOption {
	/// 获取消息冷却时间，单位为秒。
	pub fn cd(&self) -> u64 {
		self.cd.unwrap_or(default_cd())
	}

	/// 获取 Bot 别名列表
	pub fn alias(&self) -> Vec<&str> {
		self.alias.iter().flat_map(|v| v.iter().map(|s| s.as_str())).collect()
	}

	/// 获取响应模式。
	pub fn mode(&self) -> ReactiveMode {
		self.mode.unwrap_or_default()
	}
}
