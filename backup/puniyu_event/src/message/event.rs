use super::MessageBase;
use super::friend::FriendMessage;
use super::group::{GroupMessage, GroupTempMessage};
use super::guild::GuildMessage;
use crate::{
	ContactType, EventBase, EventType, SenderType, SubEventType, codegen_delegate_to_variants,
	codegen_delegate_to_variants_convert, codegen_impl_as,
};
use puniyu_bot::Bot;
use puniyu_element::receive::Elements;

/// 消息事件枚举
///
/// 统一的消息事件类型，包含好友消息、群消息、群临时消息和频道消息。
///
/// 如果只需要判断消息类别，优先使用 [`MessageBase`] 提供的
/// `is_friend()` / `is_group()` / `is_group_temp()` / `is_guild()`；
/// 如果需要获取具体消息类型，再使用
/// `as_friend()` / `as_group()` / `as_group_temp()` / `as_guild()`。
#[derive(Debug, Clone)]
pub enum MessageEvent<'m> {
	/// 好友消息事件
	Friend(FriendMessage<'m>),
	/// 群聊消息事件
	Group(GroupMessage<'m>),
	/// 群临时消息事件
	GroupTemp(GroupTempMessage<'m>),
	/// 频道消息事件
	Guild(GuildMessage<'m>),
}

codegen_impl_as! {
	MessageEvent {
		Friend(FriendMessage) => as_friend,
		Group(GroupMessage) => as_group,
		GroupTemp(GroupTempMessage) => as_group_temp,
		Guild(GuildMessage) => as_guild,
	}
}

impl<'m> EventBase for MessageEvent<'m> {
	fn time(&self) -> u64 {
		codegen_delegate_to_variants!(self, time, Friend, Group, GroupTemp, Guild)
	}

	fn event_type(&self) -> EventType {
		codegen_delegate_to_variants!(self, event_type, Friend, Group, GroupTemp, Guild)
	}

	fn event_id(&self) -> &str {
		codegen_delegate_to_variants!(self, event_id, Friend, Group, GroupTemp, Guild)
	}

	fn sub_event(&self) -> SubEventType {
		codegen_delegate_to_variants_convert!(
			self,
			sub_event,
			SubEventType,
			Friend,
			Group,
			GroupTemp,
			Guild
		)
	}

	fn bot(&self) -> &Bot {
		codegen_delegate_to_variants!(self, bot, Friend, Group, GroupTemp, Guild)
	}

	fn self_id(&self) -> &str {
		codegen_delegate_to_variants!(self, self_id, Friend, Group, GroupTemp, Guild)
	}

	fn user_id(&self) -> &str {
		codegen_delegate_to_variants!(self, user_id, Friend, Group, GroupTemp, Guild)
	}

	fn contact(&self) -> ContactType<'_> {
		codegen_delegate_to_variants!(self, contact, Friend, Group, GroupTemp, Guild)
	}

	fn sender(&self) -> SenderType<'_> {
		codegen_delegate_to_variants!(self, sender, Friend, Group, GroupTemp, Guild)
	}
}

impl<'m> MessageBase for MessageEvent<'m> {
	fn message_id(&self) -> &str {
		codegen_delegate_to_variants!(self, message_id, Friend, Group, GroupTemp, Guild)
	}

	fn elements(&self) -> &Vec<Elements<'_>> {
		codegen_delegate_to_variants!(self, elements, Friend, Group, GroupTemp, Guild)
	}
}
