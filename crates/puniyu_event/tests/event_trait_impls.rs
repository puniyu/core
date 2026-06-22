use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::{Adapter, AdapterHandle};
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, ContactType, contact_friend, contact_group_temp};
use puniyu_element::receive::Elements;
use puniyu_event::{
	EventBase, EventType, SubEventType,
	message::{FriendMessage, GroupTempMessage, MessageBase, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_sender::{Sender, SenderType, sender_friend, sender_group_temp};

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
	group_temp_contact: puniyu_contact::GroupTempContact<'static>,
	group_temp_sender: puniyu_sender::GroupTempSender<'static>,
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
		let adapter: Arc<dyn Adapter> =
			Arc::new(TestOneBotApi { adapter_info: info, account_info: account });
		Self {
			bot: Arc::new(Bot::new(AdapterHandle::new(adapter))),
			friend_contact: contact_friend!(peer: "123456", name: "Alice"),
			friend_sender: sender_friend!(user_id: "123456", nick: "Alice"),
			group_temp_contact: contact_group_temp!(peer: "654321", name: "Temp Group"),
			group_temp_sender: sender_group_temp!(user_id: "123456"),
			elements: Vec::new(),
		}
	}

	fn friend_event(&self) -> MessageEvent<'_> {
		MessageEvent::Friend(FriendMessage::new(
			self.bot.as_ref(),
			"msg-event-1",
			"123456",
			&self.friend_contact,
			&self.friend_sender,
			1,
			"msg-1",
			&self.elements,
		))
	}

	fn group_temp_event(&self) -> MessageEvent<'_> {
		MessageEvent::GroupTemp(GroupTempMessage::new(
			self.bot.as_ref(),
			"msg-event-temp-1",
			"123456",
			&self.group_temp_contact,
			&self.group_temp_sender,
			2,
			"msg-temp-1",
			&self.elements,
		))
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

fn message_summary<M>(message: &M) -> (&str, usize)
where
	M: MessageBase,
{
	(message.message_id(), message.elements().len())
}

#[test]
fn message_event_implements_event_and_message_traits() {
	let data = TestData::new();
	let event = data.friend_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::Friend));
	assert_eq!(
		base_snapshot(&event),
		(
			1,
			"msg-event-1".to_string(),
			"123456".to_string(),
			"123456".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(message_summary(&event), ("msg-1", 0));
}

#[test]
fn group_temp_message_event_implements_event_and_message_traits() {
	let data = TestData::new();
	let event = data.group_temp_event();

	assert_eq!(event.event_type(), EventType::Message);
	assert_eq!(event.sub_event(), SubEventType::Message(MessageSubEventType::GroupTemp));
	assert_eq!(
		base_snapshot(&event),
		(
			2,
			"msg-event-temp-1".to_string(),
			"123456".to_string(),
			"654321".to_string(),
			"123456".to_string(),
		)
	);
	assert_eq!(message_summary(&event), ("msg-temp-1", 0));
	assert_eq!(
		match event.sender() {
			SenderType::Friend(_) => "friend",
			SenderType::Group(_) => "group",
			SenderType::GroupTemp(_) => "grouptemp",
			SenderType::Guild(_) => "guild",
		},
		"grouptemp"
	);
	assert!(event.as_group_temp().is_some());
}
