use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// 列表配置
///
/// 用于管理白名单和黑名单的通用配置结构。
/// 可应用于群组、好友等多种场景，提供灵活的访问控制。
///
/// # 工作原理
///
/// - 白名单（`enable_list`）：只允许列表中的项通过
/// - 黑名单（`disable_list`）：阻止列表中的项通过
/// - 当两个列表都为空时，默认允许所有项通过
///
/// # 使用场景
///
/// - 群组管理：控制 Bot 可以响应哪些群聊
/// - 好友管理：控制 Bot 可以响应哪些好友消息
/// - 其他需要访问控制的场景
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_config::ListConfig;
///
/// // 在配置文件中定义
/// // [group]
/// // enable_list = ["123456", "789012"]  # 只响应这些群
/// // disable_list = ["111111"]           # 屏蔽这个群
///
/// let config = ListConfig::default();
/// let allowed_groups = config.enable_list();
/// let blocked_groups = config.disable_list();
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListConfig {
	/// 白名单列表
	///
	/// 包含允许通过的项的 ID 列表。
	/// 当此列表不为空时，只有列表中的项才会被处理。
	#[serde(default)]
	enable_list: Vec<SmolStr>,

	/// 黑名单列表
	///
	/// 包含需要屏蔽的项的 ID 列表。
	/// 列表中的项将被明确拒绝处理。
	#[serde(default)]
	disable_list: Vec<SmolStr>,
}

impl ListConfig {
	/// 获取白名单列表。
	pub fn enable_list(&self) -> Vec<&str> {
		self.enable_list.iter().map(|s| s.as_str()).collect()
	}

	/// 获取黑名单列表。
	pub fn disable_list(&self) -> Vec<&str> {
		self.disable_list.iter().map(|s| s.as_str()).collect()
	}
}
