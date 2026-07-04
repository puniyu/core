mod message;
#[doc(inline)]
pub use message::MessageContext;

use crate::BotContext;
use puniyu_bot::Bot;
use puniyu_contact::ContactType;
use puniyu_event::{Event, EventBase, EventType, SubEventType};
use puniyu_sender::SenderType;
use std::collections::HashMap;

/// 事件上下文
///
/// 提供对事件信息和命令参数的访问。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::{BotContext, EventContext};
///
/// let bot_context = BotContext::new(&bot);
/// let event_context = EventContext::new(&event, args);
///
/// // 判断事件类型
/// if event_context.is_message() {
///     if let Some(msg_ctx) = event_context.as_message() {
///         msg_ctx.reply("收到消息").await?;
///     }
/// }
///
/// // 判断场景类型
/// if event_context.is_friend() {
///     println!("好友事件");
/// } else if event_context.is_group() {
///     println!("群组事件");
/// }
/// ```
#[derive(Clone)]
pub struct EventContext<'c> {
	inner: &'c Event<'c>,
}

impl<'c> EventContext<'c> {
	/// 创建新的事件上下文
	///
	/// # 参数
	///
	/// - `event` - 事件的引用
	/// - `args` - 可选的命令参数映射
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_context = EventContext::new(&event, args);
	/// ```
	pub fn new(event: &'c Event) -> Self {
		Self { inner: event }
	}

	/// 获取当前事件关联的机器人上下文。
	pub fn as_bot(&self) -> BotContext<'_> {
		BotContext::new(self.inner.bot())
	}

	/// 尝试将当前事件上下文转换为消息上下文。
	///
	/// 当事件为消息事件时返回 [`Some`]，否则返回 [`None`]。
	pub fn as_message(&self) -> Option<MessageContext<'_>> {
		self.inner.as_message().map(|message| MessageContext::new(message, HashMap::new()))
	}

	/// 尝试获取当前事件中的扩展事件。
	pub fn as_extension(&self) -> Option<&dyn puniyu_event::ExtensionEvent> {
		self.inner.as_extension()
	}
}
impl<'c> EventBase for EventContext<'c> {
	fn time(&self) -> u64 {
		self.inner.time()
	}

	fn event_type(&self) -> EventType {
		self.inner.event_type()
	}

	fn event_id(&self) -> &str {
		self.inner.event_id()
	}

	fn sub_event(&self) -> SubEventType {
		self.inner.sub_event()
	}

	fn bot(&self) -> &Bot {
		self.inner.bot()
	}

	fn self_id(&self) -> &str {
		self.inner.self_id()
	}

	fn user_id(&self) -> &str {
		self.inner.user_id()
	}

	fn contact(&self) -> ContactType<'_> {
		self.inner.contact()
	}

	fn sender(&self) -> SenderType<'_> {
		self.inner.sender()
	}
}

impl<'e> From<&'e Event<'e>> for EventContext<'e> {
	fn from(event: &'e Event) -> Self {
		Self::new(event)
	}
}

impl<'e> From<&'e EventContext<'e>> for &'e Event<'e> {
	fn from(context: &'e EventContext<'e>) -> Self {
		context.inner
	}
}
