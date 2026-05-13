use crate::{BotContext, EventArg};
use puniyu_adapter_types::SendMsgType;
use puniyu_bot::Bot;
use puniyu_command_types::ArgValue;
use puniyu_config::app_config;
use puniyu_element::receive::Elements;
use puniyu_event::message::{
	FriendMessage, GroupMessage, GroupTempMessage, GuildMessage, MessageBase, MessageEvent,
};
use puniyu_event::{EventBase, EventType};
use puniyu_message::Message;

/// 消息上下文
///
/// 提供对消息事件的专门处理，包括消息回复、参数获取、发送者信息等。
///
/// 消息类型判断方法（如 `is_friend()`、`is_group()`、`is_group_temp()`）
/// 由 [`MessageBase`] trait 默认提供。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_context::MessageContext;
///
/// async fn handle_message(ctx: &MessageContext<'_>) {
///     // 回复消息
///     ctx.reply("Hello!").await?;
///
///     // 获取命令参数
///     if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
///         ctx.reply(format!("Hello, {}!", name)).await?;
///     }
///
///     // 判断消息类型
///     if ctx.is_group() {
///         println!("群消息");
///     }
///
///     // 获取发送者信息
///     let sender = ctx.sender();
///     println!("发送者: {}", sender.user_id());
/// }
/// ```
#[derive(Clone)]
pub struct MessageContext<'c> {
	_event: &'c MessageEvent<'c>,
	_bot: BotContext<'c>,
	_args: EventArg,
}

impl<'c> MessageContext<'c> {
	/// 创建新的消息上下文
	///
	/// # 参数
	///
	/// - `event` - 消息事件的引用
	/// - `args` - 命令参数映射
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let msg_context = MessageContext::new(&message_event, args);
	/// ```
	pub fn new(event: &'c MessageEvent, args: EventArg) -> Self {
		Self { _event: event, _bot: BotContext::new(event.bot()), _args: args }
	}

	/// 获取内部消息事件
	pub fn event(&self) -> &'c MessageEvent<'c> {
		self._event
	}

	/// 获取当前消息关联的机器人上下文。
	pub fn as_bot(&self) -> &BotContext<'_> {
		&self._bot
	}

	/// 获取好友消息引用。
	///
	/// 如需仅做消息类型判断，优先使用 [`MessageBase`] 提供的
	/// `is_friend()` / `is_group()` / `is_group_temp()`。
	///
	/// 如果当前消息为好友消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_friend(&self) -> Option<&FriendMessage<'_>> {
		self._event.as_friend()
	}

	/// 获取群消息引用。
	///
	/// 如果当前消息为群消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_group(&self) -> Option<&GroupMessage<'_>> {
		self._event.as_group()
	}

	/// 获取群临时消息引用。
	///
	/// 如果当前消息为群临时消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_group_temp(&self) -> Option<&GroupTempMessage<'_>> {
		self._event.as_group_temp()
	}

	/// 获取频道消息引用。
	///
	/// 如果当前消息为频道消息则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_guild(&self) -> Option<&GuildMessage<'_>> {
		self._event.as_guild()
	}

	/// 向当前消息对应的联系人发送回复消息。
	///
	/// 参数 `message` 支持任意可转换为 [`Message`] 的类型。
	pub async fn reply<M>(&self, message: M) -> puniyu_error::Result<SendMsgType>
	where
		M: Into<Message>,
	{
		let contact = self._event.contact();
		self._bot.send_message(&contact, message).await
	}

	/// 获取命令参数值
	///
	/// 根据参数名获取对应的参数值。
	///
	/// # 参数
	///
	/// - `name` - 参数名称
	///
	/// # 返回
	///
	/// 如果参数存在则返回 [`Some(&ArgValue)`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// // 获取字符串参数
	/// if let Some(name) = ctx.arg("name").and_then(|v| v.as_str()) {
	///     println!("Name: {}", name);
	/// }
	///
	/// // 获取整数参数
	/// if let Some(count) = ctx.arg("count").and_then(|v| v.as_int()) {
	///     println!("Count: {}", count);
	/// }
	/// ```
	pub fn arg(&self, name: impl Into<String>) -> Option<&ArgValue> {
		self._args.get(&name.into())
	}

	/// 判断当前消息发送者是否为 Bot Master。
	pub fn is_master(&self) -> bool {
		let config = app_config();
		let masters = config.masters();
		masters.contains(&self.user_id())
	}
}

impl<'c> EventBase for MessageContext<'c> {
	fn time(&self) -> u64 {
		self._event.time()
	}

	fn event_type(&self) -> EventType {
		self._event.event_type()
	}
	fn event_id(&self) -> &str {
		self._event.event_id()
	}

	fn sub_event(&self) -> puniyu_event::SubEventType {
		self._event.sub_event()
	}

	fn bot(&self) -> &Bot {
		self._event.bot()
	}

	fn self_id(&self) -> &str {
		self._event.self_id()
	}

	fn user_id(&self) -> &str {
		self._event.user_id()
	}

	fn contact(&self) -> puniyu_contact::ContactType<'_> {
		EventBase::contact(self._event)
	}

	fn sender(&self) -> puniyu_sender::SenderType<'_> {
		EventBase::sender(self._event)
	}
}

impl<'c> MessageBase for MessageContext<'c> {
	fn message_id(&self) -> &str {
		self._event.message_id()
	}

	fn elements(&self) -> &Vec<Elements<'_>> {
		self._event.elements()
	}
}

impl<'c> MessageContext<'c> {
	/// 判断消息内容是否艾特了当前机器人。
	pub fn mentions_bot(&self) -> bool {
		self.get_at().contains(&self.self_id())
	}

	/// 判断消息内容是否包含 `@全体成员`。
	pub fn mentions_everyone(&self) -> bool {
		self.elements().iter().any(|e| matches!(e, Elements::At(at) if at.is_everyone()))
	}
}
