use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::{
	AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info,
};
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_contact::{Contact, ContactType, contact_friend};
use puniyu_context::EventContext;
use puniyu_element::receive::Elements;
use puniyu_event::{
	Event, EventBase, EventType, SubEventType,
	message::{FriendMessage, MessageEvent, MessageSubEventType},
};
use puniyu_message::Message;
use puniyu_runtime::{AdapterProvider, SendMessage};
use puniyu_sender::{Sender, sender_friend};

#[derive(Debug)]
struct TestAdapterRuntime {
	adapter: AdapterInfo,
}

impl AdapterProvider for TestAdapterRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter
	}
}

#[async_trait]
impl SendMessage for TestAdapterRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
	}
}

struct TestData {
	bot: Arc<Bot>,
	friend_contact: puniyu_contact::FriendContact<'static>,
	friend_sender: puniyu_sender::FriendSender<'static>,
	elements: Vec<Elements<'static>>,
}

impl TestData {
	fn new() -> Self {
		let adapter = adapter_info!(
			name: "test-adapter",
			platform: AdapterPlatform::Other,
			protocol: AdapterProtocol::Console,
		);
		let account = AccountInfo {
			uin: "10000".to_string(),
			name: "Puniyu".to_string(),
			avatar: Bytes::new(),
		};
		Self {
			bot: Arc::new(Bot::new(Arc::new(TestAdapterRuntime { adapter }), account)),
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
