//! # puniyu_command
//!
//! 统一的 puniyu 命令库，覆盖命令定义、元信息与注册表管理场景。
//!
//! ## 特性
//!
//! - 提供 [`Command`] trait 定义命令行为
//! - 提供 `CommandRegistry` 管理命令注册与查询
//! - 复用 `puniyu_command_types` 中的参数、权限和动作类型
//! - 支持命令别名、优先级和权限控制
//!
//! ## 示例
//!
//! ```rust,ignore
//! use async_trait::async_trait;
//! use puniyu_command::{Arg, Command, CommandAction, Permission};
//! use puniyu_context::MessageContext;
//!
//! struct HelloCommand;
//!
//! #[async_trait]
//! impl Command for HelloCommand {
//!     fn name(&self) -> &'static str {
//!         "hello"
//!     }
//!
//!     fn args(&self) -> Vec<Arg> {
//!         vec![Arg::string("name").required()]
//!     }
//!
//!     fn permission(&self) -> Permission {
//!         Permission::All
//!     }
//!
//!     async fn execute(&self, _ctx: &MessageContext) -> puniyu_error::Result<CommandAction> {
//!         CommandAction::done()
//!     }
//! }
//! ```

#[cfg(feature = "registry")]
mod registry;

use async_trait::async_trait;
#[cfg(feature = "registry")]
pub use registry::CommandRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_context::MessageContext;

/// 命令行为接口。
#[async_trait]
pub trait Command: Send + Sync + 'static {
	/// 返回命令名称。
	fn name(&self) -> &str;

	/// 返回命令描述。
	fn description(&self) -> Option<&str> {
		None
	}

	/// 返回命令参数列表。
	fn args(&self) -> Vec<Arg<'_>> {
		Vec::new()
	}

	/// 返回命令优先级，默认值为 `500`。
	fn priority(&self) -> u32 {
		500
	}

	/// 返回命令别名列表。
	fn alias(&self) -> Vec<&str> {
		Vec::new()
	}

	/// 返回命令权限，默认为 [`Permission::All`]。
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行命令。
	async fn execute(&self, ctx: &MessageContext) -> puniyu_error::Result<CommandAction>;
}

impl PartialEq for dyn Command {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.description() == other.description()
			&& self.args() == other.args()
			&& self.priority() == other.priority()
			&& self.alias() == other.alias()
			&& self.permission() == other.permission()
	}
}
