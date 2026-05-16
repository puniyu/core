use puniyu_contact::ContactType;
use puniyu_element::receive::Elements;
use puniyu_sender::SenderType;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// 消息标识类型。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MessageType {
	/// 字符串消息 ID。
	Id(String),
	/// 数值消息序号。
	Seq(u64),
}

impl From<String> for MessageType {
	fn from(message_id: String) -> Self {
		Self::Id(message_id)
	}
}
impl From<&str> for MessageType {
	fn from(message_id: &str) -> Self {
		Self::Id(String::from(message_id))
	}
}

impl From<u64> for MessageType {
	fn from(message_seq: u64) -> Self {
		Self::Seq(message_seq)
	}
}

/// 发送消息结果。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMsgType {
	/// 消息 ID。
	pub message_id: String,
	/// 发送时间戳，单位为秒。
	pub time: Duration,
}

/// 完整消息信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound(deserialize = "'de: 'm"))]
pub struct MessageInfo<'m> {
	/// 消息发送时间戳，单位为秒。
	pub time: u64,
	/// 消息 ID。
	pub message_id: String,
	/// 消息序列号。
	pub message_seq: u64,
	/// 消息联系人。
	pub contact: ContactType<'m>,
	/// 消息发送者。
	pub sender: SenderType<'m>,
	/// 消息元素列表。
	pub elements: Vec<Elements<'m>>,
}
