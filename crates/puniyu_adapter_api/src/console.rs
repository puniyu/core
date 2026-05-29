use async_trait::async_trait;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_error::Result;
use puniyu_message::Message;

#[async_trait]
pub trait ConsoleAdapterApi: Send + Sync {
	/// 发送私聊消息
	async fn send_private_msg(&self, user_id: u64, message: &Message) -> Result<SendMsgType>;

	/// 发送群消息
	async fn send_group_msg(&self, group_id: u64, message: &Message) -> Result<SendMsgType>;

	// 适配器信息
	fn adapter_info(&self) -> AdapterInfo;

	// 账号信息
	fn account_info(&self) -> AccountInfo;
}
