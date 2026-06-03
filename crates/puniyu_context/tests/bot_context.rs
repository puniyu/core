use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::Adapter;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_context::BotContext;
use puniyu_message::Message;

struct TestOneBotApi {
    adapter_info: AdapterInfo,
    account_info: AccountInfo,
}

#[async_trait]
impl AdapterApi for TestOneBotApi {
    async fn send_message(&self, _contact: &ContactType<'_>, _message: &Message) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
    }
    fn adapter_info(&self) -> AdapterInfo { self.adapter_info.clone() }
    fn account_info(&self) -> AccountInfo { self.account_info.clone() }
}

impl Adapter for TestOneBotApi {}

fn make_bot_with_account(uin: &str, name: &str, avatar: Bytes) -> Arc<Bot> {
    let info = AdapterInfo::builder()
        .name("test-adapter")
        .platform(AdapterPlatform::Other)
        .protocol(AdapterProtocol::Console)
        .build();
    let account = AccountInfo { uin: uin.to_string(), name: name.to_string(), avatar };
    let adapter: Arc<dyn AdapterApi> = Arc::new(TestOneBotApi { adapter_info: info, account_info: account });
    let adapter_runtime = puniyu_runtime::AdapterRuntime::new(adapter);
    let bot_runtime = puniyu_runtime::BotRuntime::new(adapter_runtime);
    Arc::new(Bot::new(bot_runtime))
}

#[test]
fn test_bot_context_creation() {
    let bot = make_bot_with_account("bot123", "TestBot", Bytes::new());
    let context = BotContext::new(bot.as_ref());

    assert_eq!(context.account().uin, "bot123");
    assert_eq!(context.account().name, "TestBot");
}

#[test]
fn test_bot_context_with_avatar() {
    let bot = make_bot_with_account("bot123", "TestBot", Bytes::from("https://example.com/avatar.jpg"));
    let context = BotContext::new(bot.as_ref());

    assert_eq!(context.account().avatar, "https://example.com/avatar.jpg");
}
