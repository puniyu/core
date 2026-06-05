use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// 适配器配置结构
///
/// 控制哪些适配器被启用或禁用。适配器用于连接不同的消息平台或服务。
///
/// # 配置优先级
///
/// 如果同一个适配器同时出现在启用和禁用列表中，禁用列表优先级更高。
///
/// # 示例
///
/// ```toml
/// [adapter]
/// enable_list = ["console", "http"]
/// disenable_list = ["websocket"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
	/// 启用的适配器列表
	///
	/// 列出需要启用的适配器名称
	#[serde(default)]
	enable_list: Vec<SmolStr>,

	/// 禁用的适配器列表
	///
	/// 列出需要禁用的适配器名称，优先级高于启用列表
	#[serde(default)]
	disable_list: Vec<SmolStr>,
}

impl AdapterConfig {
	/// 获取启用适配器列表。
	pub fn enable_list(&self) -> Vec<&str> {
		self.enable_list.iter().map(|s| s.as_str()).collect()
	}

	/// 获取禁用适配器列表。
	pub fn disable_list(&self) -> Vec<&str> {
		self.disable_list.iter().map(|s| s.as_str()).collect()
	}
}
