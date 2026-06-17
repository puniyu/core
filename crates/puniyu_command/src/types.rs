#[doc(inline)]
pub use puniyu_command_types::*;
use std::borrow::Cow;
use crate::handle::CommandHandle;

/// 命令信息
///
/// 包含命令的元信息和构建器。
#[derive(Clone)]
pub struct CommandInfo {
	/// 所属插件 ID。
	pub plugin_id: u64,
	/// 命令构建器。
	pub handle: CommandHandle,
}

impl PartialEq for CommandInfo {
	fn eq(&self, other: &Self) -> bool {
		self.plugin_id == other.plugin_id && self.handle == other.handle
	}
}

/// 命令标识符。
pub enum CommandId<'c> {
	/// 通过索引标识。
	Id(u64),
	/// 通过名称标识。
	Name(Cow<'c, str>),
}

impl From<u64> for CommandId<'_> {
	fn from(id: u64) -> Self {
		Self::Id(id)
	}
}

impl<'c> From<&'c str> for CommandId<'c> {
	fn from(name: &'c str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for CommandId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}
