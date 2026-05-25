//! # puniyu_adapter_core
//!
//! 统一的 puniyu 适配器核心库，覆盖适配器定义与注册表管理场景。

mod registry;
use puniyu_semver::Version;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_account::AccountInfo;
use puniyu_config::Config;
use puniyu_runtime::AdapterRuntime;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
	/// 返回该适配器下所有需要注册的账号列表
	fn accounts(&self) -> Vec<AccountInfo> { Vec::new() }

	/// 获取适配器运行时。
	fn runtime(&self) -> Arc<dyn AdapterRuntime>;

	fn core_version(&self) -> Version {
		puniyu_version::VERSION
	}

	/// 获取配置列表。
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 获取服务器扩展。
	fn server(&self) -> Option<puniyu_server::ServerFunction> {
		None
	}

	/// 适配器加载时回调。
	async fn on_load(&self) -> puniyu_error::Result {
		log::info!("Adapter: loaded");
		Ok(())
	}

	/// 适配器卸载时回调。
	async fn on_unload(&self) -> puniyu_error::Result {
		log::info!("Adapter: unloaded");
		Ok(())
	}
}

impl PartialEq for dyn Adapter {
	fn eq(&self, other: &Self) -> bool {
		self.runtime().adapter_info() == other.runtime().adapter_info()
	}
}
