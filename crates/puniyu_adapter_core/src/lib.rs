//! # puniyu_adapter_core
//!
//! 统一的 puniyu 适配器核心库，覆盖适配器定义与注册表管理场景。

mod registry;
use puniyu_version::Version;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_config::Config;
use puniyu_hook::Hook;
use puniyu_runtime::AdapterRuntime;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
	/// 获取适配器运行时。
	fn runtime(&self) -> Arc<dyn AdapterRuntime>;

	fn core_version(&self) -> Version {
		Version {
			major: const_str::parse!(env!("CORE_VERSION_MAJOR"), u64),
			minor: const_str::parse!(env!("CORE_VERSION_MINOR"), u64),
			patch: const_str::parse!(env!("CORE_VERSION_PATCH"), u64),
		}
	}

	/// 获取配置列表。
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 获取钩子列表。
	fn hook(&self) -> Vec<Arc<dyn Hook>> {
		Vec::new()
	}

	/// 获取服务器扩展。
	fn server(&self) -> Option<puniyu_server::ServerFunction> {
		None
	}

	/// 初始化适配器。
	async fn init(&self) -> puniyu_error::Result {
		log::info!("Adapter: 初始化完成");
		Ok(())
	}
}

impl PartialEq for dyn Adapter {
	fn eq(&self, other: &Self) -> bool {
		self.runtime().adapter_info() == other.runtime().adapter_info()
	}
}
