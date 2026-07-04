use puniyu_contact::{Contact, GroupTempContact};
use puniyu_sender::{GroupTempSender, Role};

use super::super::MessageSubEventType;

super::super::codegen_message! {
	/// 群临时消息事件
	///
	/// 表示从群临时会话接收到的消息事件。
	GroupTempMessage, GroupTempContact, GroupTempSender, MessageSubEventType::GroupTemp
}

impl GroupTempMessage<'_> {
	/// 获取群 ID
	pub fn group_id(&self) -> &str {
		self.contact.peer()
	}

	/// 判断发送者是否为管理员
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role(), Role::Admin)
	}

	/// 判断发送者是否为群主
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role(), Role::Owner)
	}
}
