//! # puniyu_bot
//!
//! 统一的机器人门面与全局注册表。
//!
//! ## 特性
//!
//! - 提供 `Bot` trait
//! - 提供 `BotRegistry` 与 `BotId`
//! - 提供便捷函数 `get_bot`、`get_bot_count` 与 `get_all_bot`
//! - 提供宏 `register_bot!` 与 `unregister_bot!`

mod registry;
use log::debug;
#[doc(inline)]
pub use registry::BotRegistry;
mod macros;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_account::AccountInfo;
use puniyu_adapter_types::AdapterInfo;
use puniyu_contact::{Contact, ContactType};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_message::Message;
pub use puniyu_runtime::{AdapterRuntime, Runtime};
use std::sync::Arc;

#[derive(Clone)]
pub struct Bot {
	adapter: Arc<dyn AdapterRuntime>,
	account: AccountInfo,
}

impl Bot {
	pub fn new(adapter: Arc<dyn AdapterRuntime>, account: AccountInfo) -> Self {
		Self { adapter, account }
	}

	/// 获取适配器 Runtime
	pub fn runtime<T: Runtime>(&self) -> Option<&T> {
		let runtime: &dyn Runtime = self.adapter.as_ref();
		runtime.downcast_ref()
	}

	/// 返回适配器信息。
	pub fn adapter_info(&self) -> &AdapterInfo {
		self.adapter.adapter_info()
	}

	/// 返回账户信息。
	pub fn account_info(&self) -> &AccountInfo {
		&self.account
	}

	/// 发送消息。
	pub async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> puniyu_error::Result<puniyu_adapter_types::SendMsgType> {
		let (msg_type, user_id) = match contact {
			ContactType::Friend(friend) => ("PrivateMessage", &friend.peer()),
			ContactType::Group(group) => ("GroupMssage", &group.peer()),
			ContactType::GroupTemp(group) => ("Group TempMessage", &group.peer()),
			ContactType::Guild(guild) => ("GuildMessage", &guild.peer()),
		};
		debug!("[{}:{}]\n{:#?}", format!("Send {}", msg_type).yellow(), user_id.green(), message);
		self.adapter.send_message(contact, message).await
	}
}

impl std::fmt::Debug for Bot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Bot")
			.field("adapter_info", &self.adapter_info())
			.field("account_info", &self.account_info())
			.finish()
	}
}

impl PartialEq for Bot {
	fn eq(&self, other: &Self) -> bool {
		self.adapter_info() == other.adapter_info() && self.account_info() == other.account_info()
	}
}
