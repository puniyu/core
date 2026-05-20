//! # puniyu_runtime
//!
//! puniyu 的统一运行时抽象与运行时句柄定义。
//!
//! 该 crate 主要提供两类能力：
//! - 运行时 trait 抽象，用于描述适配器、Bot 与消息发送等通用能力
//! - 运行时句柄类型，用于管理框架内部的异步运行单元
//!
//! ## 提供内容
//!
//! - [`Runtime`]：运行时基础 trait，提供只读运行时访问视图
//! - [`AdapterProvider`]：访问适配器信息
//! - [`AdapterRuntime`]：适配器级运行时抽象
//! - [`SendMessage`]：发送消息能力 trait
//! - [`ServerRuntime`]：HTTP 服务运行句柄，封装服务停止与等待结束等生命周期能力
//!
//! ## 设计说明
//!
//! trait 抽象主要服务于上层业务与插件系统，便于以统一接口访问运行时能力；
//! 具体句柄类型则用于承载框架内部异步任务的生命周期管理。
mod server;
#[doc(inline)]
pub use server::ServerRuntime;
use std::any::Any;

use async_trait::async_trait;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

pub trait Runtime: Any + Send + Sync {}

impl dyn Runtime {
	pub fn as_any(&self) -> &dyn Any {
		self
	}

	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref::<T>()
	}
}

pub trait AdapterProvider: Send + Sync {
	fn adapter_info(&self) -> &AdapterInfo;
}

pub trait AdapterRuntime: Runtime + AdapterProvider + SendMessage {}


impl<T> AdapterRuntime for T where T: Runtime + AdapterProvider + SendMessage {}

impl<T: 'static> Runtime for T where T: AdapterProvider + SendMessage {}

#[async_trait]
pub trait SendMessage: Send + Sync {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType>;
}
