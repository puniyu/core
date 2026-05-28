//! # puniyu_runtime
//!
//! puniyu 的统一运行时抽象与运行时句柄定义。
//!
//! ## 提供内容
//!
//! - [`AdapterRuntime`]：适配器级运行时结构体
//! - [`BotRuntime`]：Bot 级运行时结构体
//! - `ServerRuntime`：HTTP 服务运行句柄
//!
//! ## 设计说明
//!
//! 运行时仅做结构组合，元信息从 API 获取。

mod server;
#[doc(inline)]
pub use server::ServerRuntime;

use std::sync::Arc;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;

/// 适配器级运行时（所有 Bot 共享）。
///
/// 仅承载适配器元信息，作为 `BotRuntime` 的共享组合层。
#[derive(Clone)]
pub struct AdapterRuntime {
    info: AdapterInfo,
}

impl AdapterRuntime {
    pub fn new(info: AdapterInfo) -> Self {
        Self { info }
    }

    pub fn info(&self) -> &AdapterInfo {
        &self.info
    }
}

/// Bot 级运行时（每个账号独立）。
///
/// 组合适配器运行时与 per-account API，是 `Bot` 的核心数据结构。
#[derive(Clone)]
pub struct BotRuntime {
    adapter: AdapterRuntime,
    api: Arc<dyn AdapterApi>,
}

impl BotRuntime {
    pub fn new(adapter: AdapterRuntime, api: Arc<dyn AdapterApi>) -> Self {
        Self { adapter, api }
    }

    pub fn adapter_info(&self) -> AdapterInfo {
        self.api.adapter_info()
    }

    pub fn adapter_runtime(&self) -> &AdapterRuntime {
        &self.adapter
    }

    pub fn account_info(&self) -> AccountInfo {
        self.api.account_info()
    }

    pub fn api(&self) -> &dyn AdapterApi {
        &*self.api
    }
}
