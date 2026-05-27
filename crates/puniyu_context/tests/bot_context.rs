use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_adapter_api::{AdapterApi, OneBotAdapterApi};
use puniyu_bot::Bot;
use puniyu_context::BotContext;
use puniyu_message::Message;

struct TestOneBotApi;

#[async_trait]
impl OneBotAdapterApi for TestOneBotApi {
    async fn send_private_msg(
        &self,
        _user_id: u64,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
    }

    async fn send_group_msg(
        &self,
        _group_id: u64,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
    }
}

impl Clone for TestOneBotApi {
    fn clone(&self) -> Self {
        Self
    }
}

fn make_bot_with_account(uin: &str, name: &str, avatar: Bytes) -> Arc<Bot> {
    let info = AdapterInfo::builder()
        .name("test-adapter")
        .platform(AdapterPlatform::Other)
        .protocol(AdapterProtocol::Console)
        .build();
    let api = Arc::new(TestOneBotApi) as Arc<dyn AdapterApi>;
    let account = AccountInfo { uin: uin.to_string(), name: name.to_string(), avatar };
    let runtime = puniyu_runtime::AdapterRuntime::new(info, api);
    Arc::new(Bot::new(runtime, account))
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