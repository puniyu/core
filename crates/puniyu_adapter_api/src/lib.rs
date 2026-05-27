//! # puniyu_adapter_api
//!
//! 统一的适配器 API trait 定义。
//!
//! ## 提供内容
//!
//! - [`AdapterApi`]：适配器基础 API trait
//! - [`OneBotAdapterApi`]：OneBot 协议 API trait


use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;
use async_trait::async_trait;

mod onebot;
pub use onebot::OneBotAdapterApi;



#[async_trait]
pub trait AdapterApi: Send + Sync { 
    async fn send_message(
        &self,
        contact: &ContactType<'_>,
        message: &Message,
    ) -> Result<SendMsgType>;

}

