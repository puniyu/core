mod types;
#[doc(inline)]
pub use types::*;
#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::PluginRegistry;

use async_trait::async_trait;
use log::info;
use puniyu_command::Command;
use puniyu_config::Config;
use puniyu_error::Result;
use puniyu_server::ServerFunction;
use puniyu_task::Task;
use puniyu_version::Version;
use std::sync::Arc;

#[async_trait]
pub trait Plugin: Send + Sync + 'static {
	/// 插件名称
	fn name(&self) -> &str;
	/// 插件版本
	fn version(&self) -> Version;
	/// Core版本
	fn core_version(&self) -> Version {
		Version {
			major: const_str::parse!(env!("CORE_VERSION_MAJOR"), u64),
			minor: const_str::parse!(env!("CORE_VERSION_MINOR"), u64),
			patch: const_str::parse!(env!("CORE_VERSION_PATCH"), u64),
		}
	}
	/// 插件描述
	fn description(&self) -> Option<&str>;
	/// 插件作者
	fn author(&self) -> Vec<&str> {
		vec![]
	}

	/// 插件命令前缀
	fn prefix(&self) -> Option<&str> {
		None
	}

	/// 任务列表
	fn tasks(&self) -> Vec<Arc<dyn Task>> {
		Vec::new()
	}

	/// 命令列表
	fn commands(&self) -> Vec<Arc<dyn Command>> {
		Vec::new()
	}

	/// 插件配置文件
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 路由管理
	fn server(&self) -> Option<ServerFunction> {
		None
	}

	/// 插件加载时回调
	async fn on_load(&self) -> Result {
		info!("plugin: {} v{} loaded", self.name(), self.version());
		Ok(())
	}

	/// 插件卸载时回调
	async fn on_unload(&self) -> Result {
		info!("plugin: {} v{} unloaded", self.name(), self.version());
		Ok(())
	}
}

impl PartialEq for dyn Plugin {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.prefix() == other.prefix()
			&& self.tasks() == other.tasks()
			&& self.commands() == other.commands()
			&& self.config() == other.config()
	}
}
