use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// 插件配置结构
///
/// 控制哪些插件被启用或禁用。
///
/// # 配置优先级
///
/// 如果同一个插件同时出现在启用和禁用列表中，禁用列表优先级更高。
///
/// # 示例
///
/// ```toml
/// [plugin]
/// enable_list = ["echo", "weather"]
/// disable_list = ["debug"]
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
	/// 启用的插件列表
	#[serde(default)]
	enable_list: Vec<SmolStr>,

	/// 禁用的插件列表，优先级高于启用列表
	#[serde(default)]
	disable_list: Vec<SmolStr>,
}

impl PluginConfig {
	/// 获取启用插件列表。
	pub fn enable_list(&self) -> Vec<&str> {
		self.enable_list.iter().map(|s| s.as_str()).collect()
	}

	/// 获取禁用插件列表。
	pub fn disable_list(&self) -> Vec<&str> {
		self.disable_list.iter().map(|s| s.as_str()).collect()
	}
}
