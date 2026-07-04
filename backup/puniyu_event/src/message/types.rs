use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 消息子类型枚举
///
/// 定义消息的具体来源类型。
///
/// # 变体
///
/// - `Friend` - 好友消息，来自私聊
/// - `Group` - 群消息，来自群聊
/// - `Guild` - 频道消息，来自频道
///
/// # 示例
///
/// ## 基本使用
///
/// ```rust
/// use puniyu_event::message::MessageSubEventType;
///
/// let msg_type = MessageSubEventType::Friend;
/// assert_eq!(msg_type.to_string(), "friend");
/// ```
///
/// ## 从字符串解析
///
/// ```rust
/// use puniyu_event::message::MessageSubEventType;
/// use std::str::FromStr;
///
/// let msg_type = MessageSubEventType::from_str("group").unwrap();
/// assert_eq!(msg_type, MessageSubEventType::Group);
/// ```
///
/// ## 类型判断
///
/// ```rust,ignore
/// use puniyu_event::message::{MessageEvent, MessageSubEventType};
///
/// fn handle_message(event: &MessageEvent) {
///     match event.sub_event() {
///         MessageSubEventType::Friend => {
///             println!("处理好友消息");
///         }
///         MessageSubEventType::Group => {
///             println!("处理群消息");
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(
	Debug,
	Copy,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MessageSubEventType {
	/// 好友消息
	Friend,
	/// 群消息
	Group,
	/// 群临时消息
	GroupTemp,
	/// 频道消息
	Guild,
}
