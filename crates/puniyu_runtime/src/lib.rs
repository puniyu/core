//! # puniyu_runtime
//!
//! puniyu 的统一运行时抽象与运行时句柄定义。
//!
//! ## 提供内容
//!
//! - [`AdapterRuntime`]：适配器级运行时结构体
//! - `ServerRuntime`：HTTP 服务运行句柄
//!
//! ## 设计说明
//!
//! 通过结构体代替 trait object，提供编译期类型安全和优化。
mod server;
#[doc(inline)]
pub use server::ServerRuntime;
use std::sync::Arc;

use puniyu_adapter_types::AdapterInfo;
use puniyu_adapter_api::AdapterApi;

#[derive(Clone)]
pub struct AdapterRuntime {
    info: AdapterInfo,
    api: Arc<dyn AdapterApi>,
}

impl AdapterRuntime {
    pub fn new(info: AdapterInfo, api: Arc<dyn AdapterApi>) -> Self {
        Self { info, api }
    }

    pub fn info(&self) -> &AdapterInfo {
        &self.info
    }

    pub fn api(&self) -> &dyn AdapterApi {
        &*self.api
    }
}
