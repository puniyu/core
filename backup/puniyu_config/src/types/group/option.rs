use crate::types::{ReactiveMode, default_cd, empty_vec_as_none};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
/// 群组配置选项
///
/// 定义单个群组的配置参数。
///
/// # 配置继承
///
/// 特定群组配置会继承全局配置。如果某个字段未设置（`None`），
/// 则使用全局配置中的对应值。
///
/// # 配置项
///
/// - `cd`: 群组级冷却时间（秒），控制群组整体的响应频率
/// - `user_cd`: 用户级冷却时间（秒），控制单个用户的响应频率
/// - `mode`: 响应模式，控制 Bot 响应哪些消息
/// - `alias`: Bot 别名列表，用于命令识别
///
/// # 示例
///
/// ```toml
/// [global]
/// cd = 0
/// user_cd = 0
/// mode = 0
/// alias = []
///
/// [group.group_123]
/// # 只覆盖 cd 和 user_cd，其他继承全局配置
/// cd = 10
/// user_cd = 5
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupOption {
	/// 群组级冷却时间（秒）
	///
	/// 控制群组整体的响应频率，防止群组消息过于频繁
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	cd: Option<u64>,

	/// 用户级冷却时间（秒）
	///
	/// 控制单个用户在群组中的响应频率
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	user_cd: Option<u64>,

	/// 响应模式
	///
	/// 控制 Bot 在群组中响应哪些类型的消息
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none")]
	mode: Option<ReactiveMode>,

	/// Bot 别名列表
	///
	/// 用于命令识别，当消息以别名开头时会被识别为命令
	/// 如果未设置，继承全局配置
	#[serde(skip_serializing_if = "Option::is_none", deserialize_with = "empty_vec_as_none")]
	alias: Option<Vec<SmolStr>>,
}

impl Default for GroupOption {
	#[inline]
	fn default() -> Self {
		Self {
			cd: Some(default_cd()),
			user_cd: Some(default_cd()),
			mode: Some(Default::default()),
			alias: Some(Default::default()),
		}
	}
}

impl crate::common::MergeWith for GroupOption {
	fn merge_with(&self, global: &GroupOption) -> GroupOption {
		GroupOption {
			cd: self.cd.or(global.cd),
			user_cd: self.user_cd.or(global.user_cd),
			mode: self.mode.or(global.mode),
			alias: self.alias.clone().or(global.alias.clone()),
		}
	}
}

impl GroupOption {
	/// 获取群组级冷却时间，单位为秒。
	pub fn cd(&self) -> u64 {
		self.cd.unwrap_or(default_cd())
	}

	/// 获取用户级冷却时间，单位为秒。
	pub fn user_cd(&self) -> u64 {
		self.user_cd.unwrap_or(default_cd())
	}

	/// 获取响应模式。
	pub fn mode(&self) -> ReactiveMode {
		self.mode.unwrap_or_default()
	}

	/// 获取 Bot 别名列表的副本。
	pub fn alias(&self) -> Vec<&str> {
		self.alias.iter().flat_map(|v| v.iter().map(|s| s.as_str())).collect()
	}
}
