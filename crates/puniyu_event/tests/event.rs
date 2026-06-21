use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::Adapter;
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType, contact_friend};
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventType, SubEventType,
	extension::{ExtensionEvent, NoticeSubEventType},
	message::{FriendMessage, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_sender::{Sender, sender_friend};

struct TestOneBotApi {
	adapter_info: AdapterInfo,
	account_info: AccountInfo,
}

#[async_trait]
impl AdapterApi for TestOneBotApi {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
	}
	fn adapter_info(&self) -> AdapterInfo {
		self.adapter_info.clone()
	}
	fn account_info(&self) -> AccountInfo {
		self.account_info.clone()
	}
	async fn call_api(
		&self,
		_action: &str,
		_params: serde_json::Value,
	) -> puniyu_error::Result<puniyu_common::Response<serde_json::Value>> {
		unimplemented!("mock")
	}
}

impl Adapter for TestOneBotApi {}

struct TestData {
	bot: Arc<Bot>,
	friend_contact: puniyu_contact::FriendContact<'static>,
	friend_sender: puniyu_sender::FriendSender<'static>,
	elements: Vec<Elements<'static>>,
}

impl TestData {
	fn new() -> Self {
		let info = AdapterInfo::builder()
			.name("console")
			.platform(AdapterPlatform::QQ)
			.protocol(AdapterProtocol::Console)
			.build();
		let account = AccountInfo {
			uin: "10000".to_string(),
			name: "Puniyu".to_string(),
			avatar: Bytes::new(),
		};
		let adapter: Arc<dyn AdapterApi> =
			Arc::new(TestOneBotApi { adapter_info: info, account_info: account });
		let adapter_runtime = puniyu_runtime::AdapterRuntime::new(adapter);
		let bot_runtime = puniyu_runtime::BotRuntime::new(adapter_runtime);
		Self {
			bot: Arc::new(Bot::new(bot_runtime)),
			friend_contact: contact_friend!(peer: "123456", name: "Alice"),
			friend_sender: sender_friend!(user_id: "123456", nick: "Alice"),
			elements: Vec::new(),
		}
	}

	fn message_event(&self) -> Event<'_> {
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

	fn extension_event(&self) -> Event<'_> {
		Event::Extension(Box::new(TestExtensionEvent {
			bot: Arc::clone(&self.bot),
			contact: self.friend_contact.clone(),
			sender: self.friend_sender.clone(),
		}))
	}
}

fn event_snapshot(event: &Event<'_>) -> (u64, String, String, String, String) {
	(
		event.time(),
		event.event_id().to_string(),
		event.user_id().to_string(),
		event.contact().peer().to_string(),
		event.sender().user_id().to_string(),
	)
}

pub struct TestExtensionEvent {
	bot: Arc<Bot>,
	contact: puniyu_contact::FriendContact<'static>,
	sender: puniyu_sender::FriendSender<'static>,
}

impl puniyu_event::EventBase for TestExtensionEvent {
	fn time(&self) -> u64 {
		2
	}
	fn event_type(&self) -> puniyu_event::EventType {
		puniyu_event::EventType::Extension
	}
	fn event_id(&self) -> &str {
		"ext-event-1"
	}
	fn sub_event(&self) -> puniyu_event::SubEventType {
		self.r#type()
	}
	fn bot(&self) -> &Bot {
		self.bot.as_ref()
	}
	fn self_id(&self) -> &str {
		self.bot.self_id()
	}
	fn user_id(&self) -> &str {
		"123456"
	}
	fn contact(&self) -> puniyu_contact::ContactType<'_> {
		self.contact.clone().into()
	}
	fn sender(&self) -> puniyu_sender::SenderType<'_> {
		self.sender.clone().into()
	}
}

impl ExtensionEvent for TestExtensionEvent {
	fn r#type(&self) -> SubEventType {
		SubEventType::Notice(NoticeSubEventType::new("friend_poke"))
	}

	fn content(&self) -> &str {
		"Alice poked the bot"
	}
}

#[test]
fn root_event_message_helpers_work() {
	let data = TestData::new();
	let event = data.message_event();

	assert!(event.as_message().is_some());
	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		event_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}

#[test]
fn root_event_extension_helpers_work() {
	let data = TestData::new();
	let event = data.extension_event();

	assert!(event.as_message().is_none());
	assert!(event.as_extension().is_some());
	assert_eq!(event.event_type(), EventType::Extension);
	assert_eq!(event.sub_event(), SubEventType::Notice(NoticeSubEventType::new("friend_poke")));
	assert_eq!(
		event.as_extension().unwrap().r#type(),
		SubEventType::Notice(NoticeSubEventType::new("friend_poke"))
	);
	assert_eq!(event.as_extension().unwrap().content(), "Alice poked the bot");
	assert_eq!(
		event_snapshot(&event),
		(
			2,
			"ext-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
}
