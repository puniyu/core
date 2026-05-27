use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_adapter_api::AdapterApi;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_message::Message;

/// 机器人上下文
#[derive(Clone, Copy)]
pub struct BotContext<'c> {
    inner: &'c Bot,
}

impl<'c> BotContext<'c> {
    pub fn new(bot: &'c Bot) -> Self {
        Self { inner: bot }
    }

    pub fn api(&self) -> &dyn AdapterApi {
        self.inner.api()
    }

    pub fn adapter_info(&self) -> &AdapterInfo {
        self.inner.adapter_info()
    }

    pub async fn send_message<M>(
        &self,
        contact: &ContactType<'_>,
        message: M,
    ) -> puniyu_error::Result<SendMsgType>
    where
        M: Into<Message>,
    {
        let message = message.into();
        self.inner.send_message(contact, &message).await
    }

    pub fn account(&self) -> &AccountInfo {
        self.inner.account_info()
    }
}
