use crate::Event;
use crate::extension::{NoticeSubEventType, RequestSubEventType};
use crate::message::MessageSubEventType;
use crate::{ContactType, SenderType};
use puniyu_bot::Bot;
use puniyu_contact::{Contact, SceneType};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 事件类型枚举
///
/// 定义了所有可能的事件类型，用于区分不同类别的事件。
///
/// # 变体
///
/// - `Message` - 消息事件，包括好友消息和群消息
/// - `Notion` - 通知事件，包括戳一戳、撤回、文件上传等
/// - `Request` - 请求事件，包括好友申请、群申请等
/// - `Unknown` - 未知事件类型（默认值）
///
/// # 示例
///
/// ```rust
/// use puniyu_event::EventType;
/// use std::str::FromStr;
///
/// let event_type = EventType::Message;
/// assert_eq!(event_type.to_string(), "message");
///
/// let event_type = EventType::from_str("message").unwrap();
/// assert_eq!(event_type, EventType::Message);
/// ```
///
/// # 序列化
///
/// 该枚举实现了 `Serialize` 和 `Deserialize`，可以直接用于 JSON 序列化：
///
/// ```rust
/// use puniyu_event::EventType;
/// use serde_json;
///
/// let event_type = EventType::Message;
/// let json = serde_json::to_string(&event_type).unwrap();
/// assert_eq!(json, r#""message""#);
/// ```
#[derive(
	Debug,
	Default,
	Copy,
	Clone,
	EnumString,
	Display,
	IntoStaticStr,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Deserialize,
	Serialize,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventType {
	/// 消息事件
	Message,
	/// 扩展事件
	Extension,
	/// 未知事件类型
	#[default]
	Unknown,
}

impl<'e> From<Event<'e>> for EventType {
	fn from(event: Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Extension(_) => EventType::Extension,
		}
	}
}

impl<'e> From<&Event<'e>> for EventType {
	fn from(event: &Event) -> Self {
		match event {
			Event::Message(_) => EventType::Message,
			Event::Extension(_) => EventType::Extension,
		}
	}
}

impl<'e> std::fmt::Debug for Event<'e> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Event::Message(message_event) => f.debug_tuple("Message").field(message_event).finish(),
			Event::Extension(extension_event) => f
				.debug_struct("Extension")
				.field("sub_event", &extension_event.sub_event())
				.field("event_id", &extension_event.event_id())
				.finish(),
		}
	}
}

/// 子事件类型枚举
///
/// 统一的子事件类型枚举，包含消息、通知和请求子类型。
///
/// # 变体
///
/// - `Message(MessageSubEventType)` - 消息子类型（好友消息、群消息等）
/// - `Notice(NoticeSubEventType)` - 通知子类型
/// - `Request(RequestSubEventType)` - 请求子类型
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::{SubEventType, extension::NoticeSubEventType, message::MessageSubEventType};
///
/// let sub_event = SubEventType::Message(MessageSubEventType::Friend);
/// match sub_event {
///     SubEventType::Message(msg_type) => {
///         println!("消息类型: {:?}", msg_type);
///     }
///     SubEventType::Notice(notice_type) => {
///         println!("通知类型: {}", notice_type);
///     }
///     SubEventType::Request(request_type) => {
///         println!("请求类型: {}", request_type);
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubEventType {
	Message(MessageSubEventType),
	Notice(NoticeSubEventType),
	Request(RequestSubEventType),
}

impl From<MessageSubEventType> for SubEventType {
	fn from(sub_event: MessageSubEventType) -> Self {
		Self::Message(sub_event)
	}
}
impl From<&MessageSubEventType> for SubEventType {
	fn from(sub_event: &MessageSubEventType) -> Self {
		Self::Message(*sub_event)
	}
}

impl From<NoticeSubEventType> for SubEventType {
	fn from(sub_event: NoticeSubEventType) -> Self {
		Self::Notice(sub_event)
	}
}
impl From<&NoticeSubEventType> for SubEventType {
	fn from(sub_event: &NoticeSubEventType) -> Self {
		Self::Notice(sub_event.clone())
	}
}

impl From<RequestSubEventType> for SubEventType {
	fn from(sub_event: RequestSubEventType) -> Self {
		Self::Request(sub_event)
	}
}
impl From<&RequestSubEventType> for SubEventType {
	fn from(sub_event: &RequestSubEventType) -> Self {
		Self::Request(sub_event.clone())
	}
}

/// 事件基础 trait
///
/// 定义所有事件类型的通用接口，提供事件的基本信息访问方法。
///
/// 该 trait 是所有事件类型的基础，包括消息事件、通知事件和请求事件。
/// 通过实现此 trait，可以统一访问事件的基本属性。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_event::EventBase;
///
/// fn print_event_info<E>(event: &E)
/// where
///     E: EventBase
/// {
///     println!("事件 ID: {}", event.event_id());
///     println!("时间戳: {}", event.time());
///     println!("用户 ID: {}", event.user_id());
///     println!("机器人 ID: {}", event.self_id());
/// }
/// ```
///
/// # 实现者
///
/// - `FriendMessage` - 好友消息事件
/// - `GroupMessage` - 群消息事件
/// - 各类通知事件（戳一戳、撤回等）
/// - 各类请求事件（好友申请、群申请等）
pub trait EventBase: Send + Sync {
	/// 获取事件触发时间戳（秒）。
	fn time(&self) -> u64;

	/// 获取事件类型。
	fn event_type(&self) -> EventType;

	/// 获取事件 ID。
	fn event_id(&self) -> &str;

	/// 获取事件子类型。
	fn sub_event(&self) -> SubEventType;

	/// 获取机器人实例。
	fn bot(&self) -> &Bot;

	/// 获取机器人 ID。
	fn self_id(&self) -> &str;

	/// 获取用户 ID。
	fn user_id(&self) -> &str;

	/// 获取联系人信息。
	fn contact(&self) -> ContactType<'_>;

	/// 获取发送者信息。
	fn sender(&self) -> SenderType<'_>;

	/// 判断是否为好友消息。
	fn is_friend(&self) -> bool {
		matches!(self.contact().scene(), SceneType::Friend)
	}

	/// 判断是否为群消息。
	fn is_group(&self) -> bool {
		matches!(self.contact().scene(), SceneType::Group)
	}

	/// 判断是否为群临时消息。
	fn is_group_temp(&self) -> bool {
		matches!(self.contact().scene(), SceneType::GroupTemp)
	}

	/// 判断是否为频道消息。
	fn is_guild(&self) -> bool {
		matches!(self.contact().scene(), SceneType::Guild)
	}
}

/// 为事件枚举生成方法委托和类型转换方法
///
/// 生成 `as_*` 方法、简单委托方法和类型转换方法。
macro_rules! codegen_delegate_to_variants {
	($self:ident, $method:ident, $($variant:ident),+) => {
		match $self {
			$(Self::$variant(inner) => inner.$method(),)+
		}
	};
}
pub(crate) use codegen_delegate_to_variants;

macro_rules! codegen_delegate_to_variants_convert {
	($self:ident, $method:ident, $convert:ty, $($variant:ident),+) => {
		match $self {
			$(Self::$variant(inner) => <$convert>::from(inner.$method().clone()),)+
		}
	};
}
pub(crate) use codegen_delegate_to_variants_convert;

macro_rules! codegen_impl_as {
	($enum_name:ident { $($variant:ident($type:ident) => $method_name:ident),* $(,)? }) => {
		impl $enum_name<'_> {
			$(
				#[doc = concat!("尝试获取内部的 [`", stringify!($type), "`] 引用。")]
				#[doc = ""]
				#[doc = "如果当前事件变体匹配则返回 [`Some`]，否则返回 [`None`]。"]
				pub fn $method_name(&self) -> Option<&$type<'_>> {
					match self {
						Self::$variant(inner) => Some(inner),
						_ => None,
					}
				}
			)*
		}
	};
}
pub(crate) use codegen_impl_as;
