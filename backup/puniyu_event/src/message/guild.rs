use puniyu_contact::{Contact, GuildContact};
use puniyu_sender::{GuildSender, Role};

use super::MessageSubEventType;

super::codegen_message! {
	/// 频道消息事件
	///
	/// 表示从频道接收到的消息事件。
	GuildMessage, GuildContact, GuildSender, MessageSubEventType::Guild
}

impl GuildMessage<'_> {
	/// 获取频道 ID
	pub fn guild_id(&self) -> &str {
		self.contact.peer()
	}

	/// 判断发送者是否为频道管理员
	pub fn is_admin(&self) -> bool {
		matches!(self.sender.role(), Role::Admin)
	}

	/// 判断发送者是否为频道主
	pub fn is_owner(&self) -> bool {
		matches!(self.sender.role(), Role::Owner)
	}
}
