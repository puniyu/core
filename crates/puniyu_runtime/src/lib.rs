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

mod adapter;
#[doc(inline)]
pub use adapter::AdapterRuntime;

mod bot;
#[doc(inline)]
pub use bot::BotRuntime;
