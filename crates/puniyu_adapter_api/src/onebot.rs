use async_trait::async_trait;
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::{Contact, ContactType};
use puniyu_message::Message;
use puniyu_error::Result;

use crate::AdapterApi;

#[async_trait]
pub trait OneBotAdapterApi: Send + Sync {

    async fn send_private_msg(
        &self,
        user_id: u64,
        message: &Message,
    ) -> Result<SendMsgType>;

    async fn send_group_msg(
        &self,
        group_id: u64,
        message: &Message,
    ) -> Result<SendMsgType>;
}

#[async_trait]
impl<T: OneBotAdapterApi> AdapterApi for T {
    #[allow(clippy::unwrap_used)]
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
            _ => unimplemented!("不支持的 ContactType: {:?}", contact),
        }
    }

    fn as_onebot(&self) -> Option<&dyn OneBotAdapterApi> { Some(self) }
}