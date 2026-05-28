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

use puniyu_config::Config;
use puniyu_error::Result;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
    fn name(&self) -> &str;

    fn core_version(&self) -> Version {
        puniyu_version::VERSION
    }

    fn config(&self) -> Vec<Arc<dyn Config>> {
        Vec::new()
    }

    fn server(&self) -> Option<puniyu_server::ServerFunction> {
        None
    }

    async fn on_load(&self) -> Result {
        log::info!("Adapter: loaded");
        Ok(())
    }

    async fn on_unload(&self) -> Result {
        log::info!("Adapter: unloaded");
        Ok(())
    }
}
