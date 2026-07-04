//! # puniyu_event
//!
//! 统一的事件类型，当前仅覆盖消息场景。
//!
//! ## 概述
//!
//! `puniyu_event` 提供统一的消息事件类型定义，用于处理聊天机器人中的消息场景。
//! 当前公共事件模型仅保留消息事件。
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保事件处理的正确性
//! - 🔧 **统一接口** - 通过 trait 提供统一的事件访问接口
//! - 📦 **序列化支持** - 内置 serde 支持
//! - 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配
//! - 🔄 **聚焦消息事件** - 提供统一的消息事件抽象与访问接口
//!
//! ## 快速开始
//!
//! ### 处理事件
//!
//! ```rust,ignore
//! use puniyu_event::{Event, EventBase};
//!
//! fn handle_event(event: Event) {
//!     match event {
//!         Event::Message(msg) => {
//!             let texts = msg.get_text();
//!             println!("收到消息: {:?}", texts);
//!         }
//!     }
//! }
//! ```
//!
//! ### 处理好友消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{FriendMessage, MessageBase};
//!
//! fn handle_friend_message(msg: &FriendMessage) {
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("收到好友消息: {:?}", texts);
//!     
//!     // 获取发送者信息
//!     let sender = msg.sender();
//!     println!("发送者: {}", sender.user_id());
//! }
//! ```
//!
//! ### 处理群消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{GroupMessage, MessageBase};
//!
//! fn handle_group_message(msg: &GroupMessage) {
//!     // 获取群 ID
//!     let group_id = msg.group_id();
//!     println!("群 ID: {}", group_id);
//!     
//!     // 判断发送者是否为管理员
//!     if msg.is_admin() {
//!         println!("管理员发送的消息");
//!     }
//!     
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("群消息: {:?}", texts);
//! }
//! ```
//!
//! ### 使用 EventBase trait
//!
//! ```rust,ignore
//! use puniyu_event::EventBase;
//!
//! fn print_event_info<E>(event: &E)
//! where
//!     E: EventBase
//! {
//!     println!("事件 ID: {}", event.event_id());
//!     println!("时间戳: {}", event.time());
//!     println!("用户 ID: {}", event.user_id());
//!     println!("机器人 ID: {}", event.self_id());
//! }
//! ```

pub mod extension;
pub mod message;
mod types;
#[doc(inline)]
pub use extension::*;
#[doc(inline)]
pub use types::*;

use message::MessageEvent;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_sender::SenderType;

/// 事件枚举
///
/// 统一的事件类型，当前仅包含消息事件。
///
/// # 变体
///
/// - `Message(MessageEvent)` - 消息事件，包括好友消息和群消息
///
/// # 示例
///
/// ## 基本事件处理
///
/// ```rust,ignore
/// use puniyu_event::{Event, EventBase};
///
/// fn handle_event(event: Event) {
///     match event {
///         Event::Message(msg) => {
///             let texts = msg.get_text();
///             println!("收到消息: {:?}", texts);
///         }
///     }
/// }
/// ```
///
/// ## 使用类型判断方法
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn check_event_type(event: &Event) {
///     if let Some(msg) = event.as_message() {
///         println!("这是消息事件");
///     }
/// }
/// ```
///
/// ## 访问通用事件信息
///
/// ```rust,ignore
/// use puniyu_event::Event;
///
/// fn print_event_info(event: &Event) {
///     println!("事件 ID: {}", event.event_id());
///     println!("时间戳: {}", event.time());
///     println!("用户 ID: {}", event.user_id());
///     println!("机器人 ID: {}", event.self_id());
/// }
/// ```
pub enum Event<'e> {
	/// 消息事件
	Message(Box<MessageEvent<'e>>),
	/// 扩展事件
	Extension(Box<dyn ExtensionEvent>),
}

impl Event<'_> {
	/// 尝试将事件转换为消息事件。
	pub fn as_message(&self) -> Option<&MessageEvent<'_>> {
		match self {
			Event::Message(message) => Some(message),
			Event::Extension(_) => None,
		}
	}

	/// 尝试将事件转换为扩展事件。
	pub fn as_extension(&self) -> Option<&dyn ExtensionEvent> {
		match self {
			Event::Message(_) => None,
			Event::Extension(extension) => Some(extension.as_ref()),
		}
	}
}

impl Event<'_> {
	/// 获取事件触发时间戳
	///
	/// # 返回值
	///
	/// 返回 Unix 时间戳（秒）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let timestamp = event.time();
	/// println!("事件发生时间: {}", timestamp);
	/// ```
	pub fn time(&self) -> u64 {
		match self {
			Self::Message(event) => event.time(),
			Self::Extension(event) => event.time(),
		}
	}

	/// 获取事件类型
	///
	/// # 返回值
	///
	/// 返回事件类型枚举的引用。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::EventType;
	///
	/// match event.event_type() {
	///     EventType::Message => println!("消息事件"),
	///     _ => {}
	/// }
	/// ```
	pub fn event_type(&self) -> EventType {
		match self {
			Self::Message(event) => event.event_type(),
			Self::Extension(_) => EventType::Extension,
		}
	}

	/// 获取事件 ID
	///
	/// # 返回值
	///
	/// 返回事件的唯一标识符
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_id = event.event_id();
	/// println!("事件 ID: {}", event_id);
	/// ```
	pub fn event_id(&self) -> &str {
		match self {
			Self::Message(event) => event.event_id(),
			Self::Extension(event) => event.event_id(),
		}
	}

	/// 获取事件子类型
	///
	/// # 返回值
	///
	/// 返回统一的子事件类型枚举
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_event::SubEventType;
	///
	/// let sub_event = event.sub_event();
	/// match sub_event {
	///     SubEventType::Message(msg_type) => {
	///         println!("消息子类型: {:?}", msg_type);
	///     }
	/// }
	/// ```
	pub fn sub_event(&self) -> SubEventType {
		match self {
			Self::Message(event) => event.sub_event(),
			Self::Extension(event) => event.sub_event(),
		}
	}

	/// 获取机器人实例
	///
	/// # 返回值
	///
	/// 返回处理该事件的机器人实例引用
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot = event.bot();
	/// println!("机器人账号: {}", bot.account().uin);
	/// ```
	pub fn bot(&self) -> &Bot {
		match self {
			Self::Message(event) => event.bot(),
			Self::Extension(event) => event.bot(),
		}
	}

	/// 获取机器人 ID
	///
	/// # 返回值
	///
	/// 返回机器人的唯一标识符（通常是 QQ 号）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let bot_id = event.self_id();
	/// println!("机器人 ID: {}", bot_id);
	/// ```
	pub fn self_id(&self) -> &str {
		match self {
			Self::Message(event) => event.self_id(),
			Self::Extension(event) => event.self_id(),
		}
	}

	/// 获取用户 ID
	///
	/// # 返回值
	///
	/// 返回触发事件的用户 ID
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let user_id = event.user_id();
	/// println!("用户 ID: {}", user_id);
	/// ```
	pub fn user_id(&self) -> &str {
		match self {
			Self::Message(event) => event.user_id(),
			Self::Extension(event) => event.user_id(),
		}
	}

	/// 获取联系人信息
	///
	/// # 返回值
	///
	/// 返回事件发生的联系人信息（好友、群聊或群临时）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_contact::ContactType;
	///
	/// let contact = event.contact();
	/// match contact {
	///     ContactType::Friend(friend) => {
	///         println!("好友联系人");
	///     }
	///     ContactType::Group(group) => {
	///         println!("群聊联系人");
	///     }
	///     ContactType::GroupTemp(group_temp) => {
	///         println!("群临时联系人");
	///     }
	/// }
	/// ```
	pub fn contact(&self) -> ContactType<'_> {
		match self {
			Self::Message(event) => event.contact(),
			Self::Extension(event) => event.contact(),
		}
	}

	/// 获取发送者信息
	///
	/// # 返回值
	///
	/// 返回触发事件的发送者详细信息
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_sender::SenderType;
	///
	/// let sender = event.sender();
	/// match sender {
	///     SenderType::Friend(friend_sender) => {
	///         println!("好友发送者");
	///     }
	///     SenderType::Group(group_sender) => {
	///         println!("群成员发送者");
	///     }
	///     SenderType::GroupTemp(group_temp_sender) => {
	///         println!("群临时发送者");
	///     }
	/// }
	/// ```
	pub fn sender(&self) -> SenderType<'_> {
		match self {
			Self::Message(event) => event.sender(),
			Self::Extension(event) => event.sender(),
		}
	}
}

/// 快速构建顶层消息事件。
#[macro_export]
macro_rules! create_event {
	(Message, $message:expr $(,)?) => {
		$crate::Event::Message(Box::new($message))
	};
	(Extension, $event:expr $(,)?) => {
		$crate::Event::Extension(Box::new($event))
	};
}
