use puniyu_contact::FriendContact;
use puniyu_sender::FriendSender;

use super::MessageSubEventType;

super::codegen_message! {
	/// 好友消息事件
	///
	/// 表示从好友接收到的消息事件。
	FriendMessage, FriendContact, FriendSender, MessageSubEventType::Friend
}
