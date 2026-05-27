//! # puniyu_bot
//!
//! 统一的机器人门面与全局注册表。
//!
//! ## 特性
//!
//! - 提供 `Bot` 结构体
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
use puniyu_adapter_api::AdapterApi;
use puniyu_contact::{Contact, ContactType};
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_message::Message;
pub use puniyu_runtime::{AdapterRuntime};

#[derive(Clone)]
pub struct Bot {
    runtime: AdapterRuntime,
    account: AccountInfo,
}

impl Bot {
    pub fn new(runtime: AdapterRuntime, account: AccountInfo) -> Self {
        Self { runtime, account }
    }

    pub fn runtime(&self) -> &AdapterRuntime {
        &self.runtime
    }

    pub fn api(&self) -> &dyn AdapterApi {
        self.runtime.api()
    }

    pub fn adapter_info(&self) -> &AdapterInfo {
        self.runtime.info()
    }

    pub fn account_info(&self) -> &AccountInfo {
        &self.account
    }

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
        self.runtime.api().send_message(contact, message).await
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
