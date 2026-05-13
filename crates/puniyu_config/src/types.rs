mod app;
#[doc(inline)]
pub use app::*;
mod bot;
#[doc(inline)]
pub use bot::*;
mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;

use serde::{Deserialize, Deserializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::path::{Path, PathBuf};
pub(crate) const fn default_cd() -> u64 {
	0
}

/// 反序列化时将空 Vec 转为 None
fn empty_vec_as_none<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
	D: Deserializer<'de>,
	T: Deserialize<'de>,
{
	Ok(Option::<Vec<T>>::deserialize(deserializer)?.filter(|value| !value.is_empty()))
}



/// Bot 响应模式枚举
///
/// 定义 Bot 在不同场景下的响应行为。
///
/// # 示例
///
/// ```rust
/// use puniyu_config::ReactiveMode;
///
/// let mode = ReactiveMode::AtBot;
/// match mode {
///     ReactiveMode::All => println!("响应所有消息"),
///     ReactiveMode::AtBot => println!("仅响应 @ Bot 的消息"),
///     ReactiveMode::Alias => println!("仅响应使用别名的消息"),
///     ReactiveMode::AtOrAlias => println!("响应 @ Bot 或使用别名的消息"),
///     ReactiveMode::Master => println!("仅响应主人的消息"),
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReactiveMode {
	/// 响应所有消息
	///
	/// Bot 会响应所有接收到的消息，不做任何过滤
	#[default]
	All = 0,

	/// 仅响应 @ Bot 的消息
	///
	/// 只有当消息中包含 @ Bot 时才会响应
	AtBot = 1,

	/// 仅响应使用别名的消息
	///
	/// 只有当消息以配置的别名开头时才会响应
	Alias = 2,

	/// 响应 @ Bot 或使用别名的消息
	///
	/// 当消息包含 @ Bot 或以别名开头时都会响应
	AtOrAlias = 3,

	/// 仅响应主人的消息
	///
	/// 只响应配置中定义的主人发送的消息
	Master = 4,
}

/// 配置标识符
///
/// 用于标识配置的方式，可以通过索引或路径
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigId {
	/// 通过索引标识
	Index(u64),
	/// 通过路径标识
	Path(PathBuf),
}

impl From<u64> for ConfigId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<PathBuf> for ConfigId {
	fn from(path: PathBuf) -> Self {
		Self::Path(path)
	}
}

impl From<&Path> for ConfigId {
	fn from(path: &Path) -> Self {
		Self::Path(path.to_path_buf())
	}
}

impl From<&str> for ConfigId {
	fn from(path: &str) -> Self {
		Self::Path(PathBuf::from(path))
	}
}

