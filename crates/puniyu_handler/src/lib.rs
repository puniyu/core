//! # puniyu_handler
//!
//! 事件处理器库，提供统一的 `Handler` 接口。
//!
//! ## 特性
//!
//! - `Handler` trait 定义事件处理模型
//! - 支持处理 `puniyu_event::Event`
//! - 可选 `registry` 功能用于处理器注册管理

mod types;
#[doc(inline)]
pub use types::*;
#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
#[doc(inline)]
pub use registry::HandlerRegistry;

use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_event::Event;

/// 事件处理器接口。
#[async_trait]
pub trait Handler: Send + Sync + 'static {
	/// 获取处理器名称。
	fn name(&self) -> &'static str;

	/// 获取处理优先级（值越小优先级越高）。
	fn priority(&self) -> u32 {
		5
	}

	/// 处理事件。
	async fn handle(&self, event: &Event) -> Result;
}

impl PartialEq for dyn Handler {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name() && self.priority() == other.priority()
	}
}
