//! # puniyu_adapter_api
//!
//! 统一的适配器 API trait 定义，描述协议层核心接口。
//!
//! ## 提供内容
//!
//! - [`AdapterApi`]：适配器基础 API trait，定义消息发送与元信息访问
//! - [`OneBotAdapterApi`]：OneBot 协议 API trait，支持私聊与群消息
//!
//! ## 设计说明
//!
//! 每个 API 实例自包含适配器信息（[`AdapterApi::adapter_info`]）与账号信息（[`AdapterApi::account_info`]），
//! 实现 `OneBotAdapterApi` 后自动获得 `AdapterApi` 实现。
//!

use std::any::Any;

use async_trait::async_trait;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_common::Response;
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

#[async_trait]
pub trait AdapterApi: Any + Send + Sync {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType>;

	fn adapter_info(&self) -> AdapterInfo;

	fn account_info(&self) -> AccountInfo;

	/// 调用适配器 API
	/// 
	/// ## 参数
	/// - action: 以协议开头格式为协议_action, 例如onebot_send_msg
	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> Result<Response<serde_json::Value>>;
}
