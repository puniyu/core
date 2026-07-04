mod temp;
pub use temp::GroupTempMessage;

use puniyu_contact::{Contact, GroupContact};
use puniyu_sender::{GroupSender, Role};

use super::MessageSubEventType;

super::codegen_message! {
	/// 群消息事件
	///
	/// 表示从群聊接收到的消息事件。
	GroupMessage, GroupContact, GroupSender, MessageSubEventType::Group
}

impl GroupMessage<'_> {
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
