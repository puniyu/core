use async_trait::async_trait;
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::{Contact, ContactType};
use puniyu_error::Result;
use puniyu_message::Message;

use crate::AdapterApi;
use anyhow::anyhow;

#[async_trait]
pub trait OneBotAdapterApi: Send + Sync {
    /// 发送私聊消息
	async fn send_private_msg(&self, user_id: u64, message: &Message) -> Result<SendMsgType>;

    /// 发送群消息
	async fn send_group_msg(&self, group_id: u64, message: &Message) -> Result<SendMsgType>;
}

#[async_trait]
impl<T: OneBotAdapterApi> AdapterApi for T {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType> {
		match contact {
			ContactType::Friend(contact) => {
				self.send_private_msg(contact.peer().parse::<u64>()?, message).await
			}
			ContactType::Group(contact) => {
				self.send_group_msg(contact.peer().parse::<u64>()?, message).await
			}
			ContactType::GroupTemp(concat) => {
				self.send_private_msg(concat.peer().parse::<u64>()?, message).await
			}
			_ => Err(anyhow!("unsupported contact type: {:?}", contact).into()),
		}
	}

	fn as_onebot(&self) -> Option<&dyn OneBotAdapterApi> {
		Some(self)
	}
}
