//! 消息事件模块
//!
//! 提供好友消息、群聊消息、群临时消息和频道消息的事件类型定义。

mod friend;
#[doc(inline)]
pub use friend::FriendMessage;
mod group;
#[doc(inline)]
pub use group::{GroupMessage, GroupTempMessage};
mod guild;
#[doc(inline)]
pub use guild::GuildMessage;
mod event;
#[doc(inline)]
pub use event::MessageEvent;
mod types;
use super::EventBase;
use bytes::Bytes;
use puniyu_element::receive::Elements;
#[doc(inline)]
pub use types::*;

/// 消息基础 trait
///
/// 定义所有消息事件的通用接口，提供消息内容访问和判断方法。
///
/// 该 trait 继承自 `EventBase`，因此也可以访问所有事件的基础信息。
///
/// 对于仅做消息类型判断的场景，优先使用此 trait 提供的：
/// - [`MessageBase::is_friend`]
/// - [`MessageBase::is_group`]
/// - [`MessageBase::is_group_temp`]
///
/// 如需提取具体消息类型，再使用 `MessageEvent::as_xxx()` 等接口。
pub trait MessageBase: Send + Sync + EventBase {
	/// 获取消息 ID
	fn message_id(&self) -> &str;

	/// 获取消息元素列表
	fn elements(&self) -> &Vec<Elements<'_>>;

	/// 获取所有文本元素的内容
	fn get_text(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Text(text) => Some(text.text),
				_ => None,
			})
			.collect::<Vec<&str>>()
	}

	/// 获取所有艾特元素的目标 ID
	fn get_at(&self) -> Vec<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::At(at) => Some(at.target_id),
				_ => None,
			})
			.collect::<Vec<&str>>()
	}

	/// 获取第一个图片元素
	fn get_image(&self) -> Option<&Bytes> {
		self.elements().iter().find_map(|e| match e {
			Elements::Image(image) => Some(&image.file),
			_ => None,
		})
	}

	/// 获取第一个语音元素
	fn get_record(&self) -> Option<&Bytes> {
		self.elements().iter().find_map(|e| match e {
			Elements::Record(record) => Some(&record.file),
			_ => None,
		})
	}

	/// 获取回复消息的 ID
	fn get_reply_id(&self) -> Option<&str> {
		self.elements()
			.iter()
			.filter_map(|e| match e {
				Elements::Reply(reply) => Some(reply.message_id),
				_ => None,
			})
			.next()
	}
}

/// 快速构建好友消息事件。
#[macro_export]
macro_rules! crate_friend_message {
	(
		bot: $bot:expr,
		event_id: $event_id:expr,
		user_id: $user_id:expr,
		contact: $contact:expr,
		sender: $sender:expr,
		time: $time:expr,
		message_id: $message_id:expr,
		elements: $elements:expr $(,)?
	) => {
		$crate::message::FriendMessage::new(
			$bot,
			$event_id,
			$user_id,
			$contact,
			$sender,
			$time,
			$message_id,
			$elements,
		)
	};
}

/// 快速构建群消息事件。
#[macro_export]
macro_rules! crate_group_message {
	(
		bot: $bot:expr,
		event_id: $event_id:expr,
		user_id: $user_id:expr,
		contact: $contact:expr,
		sender: $sender:expr,
		time: $time:expr,
		message_id: $message_id:expr,
		elements: $elements:expr $(,)?
	) => {
		$crate::message::GroupMessage::new(
			$bot,
			$event_id,
			$user_id,
			$contact,
			$sender,
			$time,
			$message_id,
			$elements,
		)
	};
}

/// 快速构建群临时消息事件。
#[macro_export]
macro_rules! crate_group_temp_message {
	(
		bot: $bot:expr,
		event_id: $event_id:expr,
		user_id: $user_id:expr,
		contact: $contact:expr,
		sender: $sender:expr,
		time: $time:expr,
		message_id: $message_id:expr,
		elements: $elements:expr $(,)?
	) => {
		$crate::message::GroupTempMessage::new(
			$bot,
			$event_id,
			$user_id,
			$contact,
			$sender,
			$time,
			$message_id,
			$elements,
		)
	};
}

/// 快速构建频道消息事件。
#[macro_export]
macro_rules! crate_guild_message {
	(
		bot: $bot:expr,
		event_id: $event_id:expr,
		user_id: $user_id:expr,
		contact: $contact:expr,
		sender: $sender:expr,
		time: $time:expr,
		message_id: $message_id:expr,
		elements: $elements:expr $(,)?
	) => {
		$crate::message::GuildMessage::new(
			$bot,
			$event_id,
			$user_id,
			$contact,
			$sender,
			$time,
			$message_id,
			$elements,
		)
	};
}

/// 快速构建消息事件枚举。
#[macro_export]
macro_rules! create_message {
	(Friend, $message:expr $(,)?) => {
		$crate::message::MessageEvent::Friend($message)
	};
	(Group, $message:expr $(,)?) => {
		$crate::message::MessageEvent::Group($message)
	};
	(GroupTemp, $message:expr $(,)?) => {
		$crate::message::MessageEvent::GroupTemp($message)
	};
	(Guild, $message:expr $(,)?) => {
		$crate::message::MessageEvent::Guild($message)
	};
}

/// 生成消息事件结构体及其 EventBase、MessageBase 实现
macro_rules! codegen_message {
	(
		$(#[$meta:meta])*
		$name:ident, $contact:ident, $sender:ident, $sub_event:expr
	) => {
		$(#[$meta])*
		#[derive(Debug, Clone)]
		pub struct $name<'m> {
			bot: &'m puniyu_bot::Bot,
			event_id: &'m str,
			time: u64,
			user_id: &'m str,
			message_id: &'m str,
			elements: &'m Vec<puniyu_element::receive::Elements<'m>>,
			contact: &'m $contact<'m>,
			sender: &'m $sender<'m>,
		}

		impl<'m> $name<'m> {
			#[doc = concat!("使用完整参数构建 [`", stringify!($name), "`]。")]
			#[allow(clippy::too_many_arguments)]
			pub fn new(
				bot: &'m puniyu_bot::Bot,
				event_id: &'m str,
				user_id: &'m str,
				contact: &'m $contact<'m>,
				sender: &'m $sender<'m>,
				time: u64,
				message_id: &'m str,
				elements: &'m Vec<puniyu_element::receive::Elements<'m>>,
			) -> Self {
				Self {
					bot,
					event_id,
					time,
					user_id,
					message_id,
					elements,
					contact,
					sender,
				}
			}
		}

		impl<'e> $crate::EventBase for $name<'e> {
			fn time(&self) -> u64 { self.time }
			fn event_type(&self) -> $crate::EventType { $crate::EventType::Message }
			fn event_id(&self) -> &str { self.event_id }
			fn sub_event(&self) -> $crate::SubEventType { $crate::SubEventType::Message($sub_event) }
			fn bot(&self) -> &puniyu_bot::Bot { self.bot }
			fn self_id(&self) -> &str { self.bot.self_id() }
			fn user_id(&self) -> &str { self.user_id }
			fn contact(&self) -> puniyu_contact::ContactType<'_> { self.contact.clone().into() }
			fn sender(&self) -> puniyu_sender::SenderType<'_> { self.sender.clone().into() }
		}

		impl<'m> $crate::message::MessageBase for $name<'m> {
			fn message_id(&self) -> &str { self.message_id }
			fn elements(&self) -> &Vec<puniyu_element::receive::Elements<'_>> { self.elements }
		}
	};
}
pub(crate) use codegen_message;
