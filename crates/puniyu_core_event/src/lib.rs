use std::fmt::Display;

use puniyu_core_bot::Bot;
use puniyu_core_contact::Contact;
use puniyu_core_sender::Sender;

pub trait EventBase: Send + Sync + PartialEq + Eq {
	type Bot: Bot;
	type Contact: Contact;
	type Sender: Sender;
	type EventType: Copy + Display + PartialEq + Eq;
	type SubEventType: Copy + Display + PartialEq + Eq;

	/// 获取事件触发时间戳（秒）
	fn time(&self) -> u64;

	/// 获取事件类型。
	fn event_type(&self) -> Self::EventType;

	/// 获取事件 ID。
	fn event_id(&self) -> &str;

	/// 获取事件子类型。
	fn sub_event(&self) -> Self::SubEventType;

	/// 获取机器人实例。
	fn bot(&self) -> &Self::Bot;

	/// 获取机器人ID
	fn self_id(&self) -> &str {
		self.bot().id()
	}

	/// 获取用户ID
	fn user_id(&self) -> &str;

	/// 获取联系人信息
	fn contact(&self) -> Self::Contact;

	/// 获取发送者信息
	fn sender(&self) -> Self::Sender;
}
