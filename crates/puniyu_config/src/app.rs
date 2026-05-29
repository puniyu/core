use crate::types::ListConfig;
use crate::{AdapterConfig, LoggerConfig, PluginConfig, ServerConfig};
use puniyu_common::read_config;
use puniyu_path::config_dir;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::path::PathBuf;
use std::sync::LazyLock;

static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| config_dir().join("app.toml"));

fn default_master() -> Vec<SmolStr> {
	vec![SmolStr::new("console")]
}

fn default_prefix() -> Option<SmolStr> {
	Some(SmolStr::new("!"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
	/// 日志配置
	///
	/// 包括日志级别、文件记录、保留天数等设置
	#[serde(default)]
	logger: LoggerConfig,

	/// 服务器配置
	///
	/// 包括服务器主机地址和端口号
	#[serde(default)]
	server: ServerConfig,

	/// 适配器配置
	///
	/// 控制启用哪些适配器（控制台、服务器等）
	#[serde(default)]
	adapter: AdapterConfig,

	/// 插件配置
	///
	/// 控制启用哪些插件
	#[serde(default)]
	plugin: PluginConfig,

	/// 应用级群组配置
	///
	/// 包含群聊黑白名单等全局设置
	#[serde(default)]
	group: ListConfig,

	/// 应用级好友配置
	///
	/// 包含好友黑白名单等全局设置
	#[serde(default)]
	friend: ListConfig,

	/// Bot 主人列表
	///
	/// 定义哪些用户是 Bot 的主人，拥有最高权限
	#[serde(default = "default_master")]
	masters: Vec<SmolStr>,

	/// 全局命令前缀
	///
	/// 用于识别命令的前缀字符，默认为 "!"
	#[serde(default = "default_prefix")]
	prefix: Option<SmolStr>,
}

impl Default for AppConfig {
	#[inline]
	fn default() -> Self {
		Self {
			logger: Default::default(),
			server: Default::default(),
			adapter: Default::default(),
			plugin: Default::default(),
			masters: default_master(),
			prefix: default_prefix(),
			group: Default::default(),
			friend: Default::default(),
		}
	}
}

impl AppConfig {
	/// 获取当前应用配置。
	pub fn get() -> Self {
		use crate::ConfigRegistry;
		ConfigRegistry::get(CONFIG_PATH.as_path()).and_then(|v| v.try_into().ok()).unwrap_or_else(
			|| read_config::<Self>(config_dir().as_path(), "app").unwrap_or_default(),
		)
	}

	/// 获取日志配置。
	pub fn logger(&self) -> &LoggerConfig {
		&self.logger
	}

	/// 获取服务配置。
	pub fn server(&self) -> &ServerConfig {
		&self.server
	}

	/// 获取适配器配置。
	pub fn adapter(&self) -> &AdapterConfig {
		&self.adapter
	}

	/// 获取插件配置。
	pub fn plugin(&self) -> &PluginConfig {
		&self.plugin
	}

	/// 获取应用级群组名单配置。
	pub fn group(&self) -> &ListConfig {
		&self.group
	}

	/// 获取应用级好友名单配置。
	pub fn friend(&self) -> &ListConfig {
		&self.friend
	}

	/// 获取框架主人列表
	pub fn masters(&self) -> Vec<&str> {
		self.masters.iter().map(|s| s.as_str()).collect()
	}

	/// 获取全局命令前缀
	pub fn prefix(&self) -> Option<&str> {
		self.prefix.as_deref()
	}
}

impl crate::Config for AppConfig {
	fn name(&self) -> &str {
		"app"
	}

	fn to_value(&self) -> toml::Value {
		crate::serialize_to_value(self)
	}
}
