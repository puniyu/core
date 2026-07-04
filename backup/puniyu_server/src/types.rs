mod server;
#[doc(inline)]
pub use server::ServerFunction;
mod json;
pub(crate) use json::PrettyJson;

use puniyu_common::source::SourceType;
use std::fmt::Debug;

#[derive(Clone)]
pub struct ServerInfo {
	/// 服务器来源类型
	pub source: SourceType,
	/// 服务器配置构建器
	pub builder: ServerFunction,
}

impl PartialEq for ServerInfo {
	fn eq(&self, other: &Self) -> bool {
		self.source == other.source
	}
}

impl Debug for ServerInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ServerInfo").field("source", &self.source).finish()
	}
}

pub enum ServerId {
	/// 通过索引标识服务器
	Index(u64),
	/// 通过来源类型标识服务器
	Source(SourceType),
}

impl From<u64> for ServerId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<SourceType> for ServerId {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}
