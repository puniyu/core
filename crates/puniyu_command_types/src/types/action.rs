//! 命令执行动作。

use puniyu_error::Result;
use serde::{Deserialize, Serialize};

/// 命令执行动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum CommandAction {
	/// 处理完成，停止传播。
	Done,
	/// 继续向下传播。
	Continue,
}

impl CommandAction {
	/// 返回 `Done` 动作结果。
	pub const fn done() -> Result<CommandAction> {
		Ok(CommandAction::Done)
	}

	/// 返回 `Continue` 动作结果。
	pub const fn r#continue() -> Result<CommandAction> {
		Ok(CommandAction::Continue)
	}
}

impl From<()> for CommandAction {
	fn from(_: ()) -> Self {
		CommandAction::Done
	}
}

impl From<bool> for CommandAction {
	fn from(value: bool) -> Self {
		match value {
			true => CommandAction::Done,
			_ => CommandAction::Continue,
		}
	}
}
