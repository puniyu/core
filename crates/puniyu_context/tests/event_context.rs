use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{
    AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType,
};
use puniyu_adapter_api::{AdapterApi, ConsoleAdapterApi, OneBotAdapterApi};
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::{Contact, contact_friend};
use puniyu_context::EventContext;
use puniyu_element::receive::Elements;
use puniyu_event::{
    Event, EventBase, EventType, SubEventType,
    message::{FriendMessage, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_sender::{Sender, sender_friend};

struct TestOneBotApi {
    adapter_info: AdapterInfo,
    account_info: AccountInfo,
}

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

    fn adapter_info(&self) -> AdapterInfo { self.adapter_info.clone() }
    fn account_info(&self) -> AccountInfo { self.account_info.clone() }
}

#[async_trait]
impl AdapterApi for TestOneBotApi {
    async fn send_message(&self, contact: &puniyu_contact::ContactType<'_>, message: &Message) -> puniyu_error::Result<SendMsgType> {
        match contact {
            puniyu_contact::ContactType::Friend(c) => self.send_private_msg(c.peer().parse()?, message).await,
            puniyu_contact::ContactType::Group(c) => self.send_group_msg(c.peer().parse()?, message).await,
            puniyu_contact::ContactType::GroupTemp(c) => self.send_private_msg(c.peer().parse()?, message).await,
            _ => Err(Box::new(std::io::Error::other("unsupported contact type"))),
        }
    }
    fn adapter_info(&self) -> AdapterInfo { OneBotAdapterApi::adapter_info(self) }
    fn account_info(&self) -> AccountInfo { OneBotAdapterApi::account_info(self) }
    fn as_console(&self) -> Option<&dyn ConsoleAdapterApi> { None }
    fn as_onebot(&self) -> Option<&dyn OneBotAdapterApi> { Some(self) }
}

struct TestData {
    bot: Arc<Bot>,
    friend_contact: puniyu_contact::FriendContact<'static>,
    friend_sender: puniyu_sender::FriendSender<'static>,
    elements: Vec<Elements<'static>>,
}

impl TestData {
    fn new() -> Self {
        let info = AdapterInfo::builder()
            .name("test-adapter")
            .platform(AdapterPlatform::Other)
            .protocol(AdapterProtocol::Console)
            .build();
        let account = AccountInfo {
            uin: "10000".to_string(),
            name: "Puniyu".to_string(),
            avatar: Bytes::new(),
        };
        let api = Arc::new(TestOneBotApi { adapter_info: info.clone(), account_info: account });
        let adapter_runtime = puniyu_runtime::AdapterRuntime::new(info);
        let bot_runtime = puniyu_runtime::BotRuntime::new(adapter_runtime, api);
        Self {
            bot: Arc::new(Bot::new(bot_runtime)),
            friend_contact: contact_friend!(peer: "123456", name: "Alice"),
            friend_sender: sender_friend!(user_id: "123456", nick: "Alice"),
            elements: Vec::new(),
        }
    }

    fn event(&self) -> Event<'_> {
        Event::Message(Box::new(MessageEvent::Friend(FriendMessage::new(
            self.bot.as_ref(),
            "msg-event-1",
            "123456",
            &self.friend_contact,
            &self.friend_sender,
            1,
            "msg-1",
            &self.elements,
        ))))
    }
}

fn base_snapshot<E>(event: &E) -> (u64, String, String, String, String)
where
    E: EventBase,
{
    (
        event.time(),
        event.event_id().to_string(),
        event.user_id().to_string(),
        event.contact().peer().to_string(),
        event.sender().user_id().to_string(),
    )
}

#[test]
fn event_context_implements_event_base() {
    let data = TestData::new();
    let event = data.event();
    let _args = HashMap::<String, ArgValue>::new();
    let ctx: EventContext<'_> = EventContext::new(&event);

    assert_eq!(ctx.event_type(), EventType::Message);
    assert_eq!(ctx.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
    assert_eq!(
        base_snapshot(&ctx),
        (
            1,
            "msg-event-1".to_string(),
            "123456".to_string(),
            "123456".to_string(),
            "123456".to_string(),
        )
    );
}