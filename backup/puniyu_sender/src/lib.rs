//! # puniyu_sender
//!
//! 统一的消息发送者类型，覆盖好友、群聊、群临时和频道场景。
//!
//! ## 特性
//!
//! - 提供 `FriendSender`、`GroupSender`、`GroupTempSender` 与 `GuildSender`
//! - 提供统一接口 `Sender`
//! - 提供统一枚举 `SenderType`
//! - 提供构建宏 `sender!`、`sender_friend!`、`sender_group!`、`sender_group_temp!` 和 `sender_guild!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_sender::{sender, Sender};
//!
//! let friend = sender!(Friend, user_id: "123456", nick: "Alice");
//! let group = sender!(Group, user_id: "789012", nick: "Bob");
//! let group_temp = sender!(GroupTemp, user_id: "789012", nick: "Bob");
//! let guild = sender!(Guild, user_id: "9527", nick: "Carol", card: "频道管理员");
//!
//! assert!(friend.is_friend());
//! assert_eq!(friend.user_id(), "123456");
//!
//! assert!(group.is_group());
//! assert!(group_temp.is_group_temp());
//! assert!(guild.is_guild());
//! ```

mod friend;
#[doc(inline)]
pub use friend::*;
mod group;
#[doc(inline)]
pub use group::*;
mod guild;
#[doc(inline)]
pub use guild::*;
mod types;
#[doc(inline)]
pub use types::*;

use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// 统一发送者枚举
///
/// 统一的发送者类型，可以是好友发送者或群聊发送者。
///
/// # 示例
///
/// ```rust
/// use puniyu_sender::{FriendSender, GroupSender, GroupTempSender, Role, SenderType, Sex};
///
/// // 创建好友发送者
/// let friend = FriendSender::new("123456", Some("Alice"), Sex::Female, Some(25));
/// let sender = SenderType::Friend(friend);
///
/// // 创建群聊发送者
/// let group = GroupSender::new(
///     "789012",
///     Some("Bob"),
///     Sex::Male,
///     Some(30),
///     Role::Admin,
///     Some("管理员"),
///     Some(10),
///     Some("活跃成员"),
/// );
/// let sender = SenderType::Group(group);
///
/// // 创建群临时发送者
/// let group_temp = GroupTempSender::new(
///     "246810",
///     Some("Carol"),
///     Sex::Female,
///     Some(22),
///     Role::Member,
///     Option::<&str>::None,
///     None,
///     Option::<&str>::None,
/// );
/// let sender = SenderType::GroupTemp(group_temp);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 's"))]
pub enum SenderType<'s> {
	/// 好友发送者
	Friend(FriendSender<'s>),
	/// 群聊发送者
	Group(GroupSender<'s>),
	/// 群临时发送者
	GroupTemp(GroupTempSender<'s>),
	/// 频道发送者
	Guild(GuildSender<'s>),
}

impl SenderType<'_> {
	/// 获取好友发送者引用
	///
	/// 如果当前是好友发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_friend, Sender, SenderType};
	///
	/// let sender = SenderType::from(sender_friend!(user_id: "123456", nick: "Alice"));
	///
	/// if let Some(friend) = sender.as_friend() {
	///     assert_eq!(friend.user_id(), "123456");
	/// }
	/// ```
	pub fn as_friend(&self) -> Option<&FriendSender<'_>> {
		match self {
			SenderType::Friend(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取群聊发送者引用
	///
	/// 如果当前是群聊发送者则返回 [`Some`]，否则返回 [`None`]。
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_group, Role, SenderType};
	///
	/// let sender = SenderType::from(sender_group!(user_id: "789012", role: Role::Admin));
	///
	/// if let Some(group) = sender.as_group() {
	///     assert_eq!(group.role(), &Role::Admin);
	/// }
	/// ```
	pub fn as_group(&self) -> Option<&GroupSender<'_>> {
		match self {
			SenderType::Group(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取群临时发送者引用
	pub fn as_group_temp(&self) -> Option<&GroupTempSender<'_>> {
		match self {
			SenderType::GroupTemp(sender) => Some(sender),
			_ => None,
		}
	}

	/// 获取频道发送者引用
	pub fn as_guild(&self) -> Option<&GuildSender<'_>> {
		match self {
			SenderType::Guild(sender) => Some(sender),
			_ => None,
		}
	}

	/// 判断是否为好友发送者
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_friend, SenderType};
	///
	/// let sender = SenderType::from(sender_friend!(user_id: "123456"));
	/// assert!(sender.is_friend());
	/// ```
	pub fn is_friend(&self) -> bool {
		matches!(self, SenderType::Friend(_))
	}

	/// 判断是否为群聊发送者
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_sender::{sender_group, SenderType};
	///
	/// let sender = SenderType::from(sender_group!(user_id: "789012"));
	/// assert!(sender.is_group());
	/// ```
	pub fn is_group(&self) -> bool {
		matches!(self, SenderType::Group(_))
	}

	/// 判断是否为群临时发送者
	pub fn is_group_temp(&self) -> bool {
		matches!(self, SenderType::GroupTemp(_))
	}

	/// 判断是否为频道发送者
	pub fn is_guild(&self) -> bool {
		matches!(self, SenderType::Guild(_))
	}
}

impl<'s> Sender for SenderType<'s> {
	fn user_id(&self) -> &str {
		match self {
			SenderType::Friend(sender) => sender.user_id(),
			SenderType::Group(sender) => sender.user_id(),
			SenderType::GroupTemp(sender) => sender.user_id(),
			SenderType::Guild(sender) => sender.user_id(),
		}
	}
	fn name(&self) -> Option<&str> {
		match self {
			SenderType::Friend(sender) => sender.name(),
			SenderType::Group(sender) => sender.name(),
			SenderType::GroupTemp(sender) => sender.name(),
			SenderType::Guild(sender) => sender.name(),
		}
	}
	fn sex(&self) -> &Sex {
		match &self {
			SenderType::Friend(sender) => sender.sex(),
			SenderType::Group(sender) => sender.sex(),
			SenderType::GroupTemp(sender) => sender.sex(),
			SenderType::Guild(sender) => sender.sex(),
		}
	}
	fn age(&self) -> Option<u32> {
		match self {
			SenderType::Friend(sender) => sender.age(),
			SenderType::Group(sender) => sender.age(),
			SenderType::GroupTemp(sender) => sender.age(),
			SenderType::Guild(sender) => sender.age(),
		}
	}
}

impl<'s> From<FriendSender<'s>> for SenderType<'s> {
	fn from(sender: FriendSender<'s>) -> Self {
		Self::Friend(sender)
	}
}

impl<'s> From<GroupSender<'s>> for SenderType<'s> {
	fn from(sender: GroupSender<'s>) -> Self {
		Self::Group(sender)
	}
}

impl<'s> From<GroupTempSender<'s>> for SenderType<'s> {
	fn from(sender: GroupTempSender<'s>) -> Self {
		Self::GroupTemp(sender)
	}
}

impl<'s> From<GuildSender<'s>> for SenderType<'s> {
	fn from(sender: GuildSender<'s>) -> Self {
		Self::Guild(sender)
	}
}

/// 统一的发送者构建宏
///
/// 根据发送者类型（Friend 或 Group）创建相应的发送者对象，
/// 并返回统一类型 [`SenderType`]。
///
/// # 语法
///
/// ```text
/// sender!(Friend, field: value, ...)
/// sender!(Group, field: value, ...)
/// ```
///
/// # 参数
///
/// - 第一个参数：发送者类型，必须是 `Friend` 或 `Group`
/// - 后续参数：字段名和值的键值对
///   - `user_id`: 发送者 ID（必需）
///   - `nick`: 发送者昵称（可选）
///   - `sex`: 性别（可选）
///   - `age`: 年龄（可选）
/// - 当类型为 `Group` 时，还支持：
///   - `role`: 群角色（可选）
///   - `card`: 群名片（可选）
///   - `level`: 等级（可选）
///   - `title`: 专属头衔（可选）
///
/// # 示例
///
/// ## 创建好友发送者
///
/// ```rust
/// use puniyu_sender::sender;
///
/// let sender = sender!(Friend, user_id: "123456", nick: "Alice");
/// assert!(sender.is_friend());
/// ```
///
/// ## 创建群聊发送者
///
/// ```rust
/// use puniyu_sender::{sender, Role};
///
/// let sender = sender!(Group, user_id: "789012", role: Role::Admin, card: "管理员");
/// assert!(sender.is_group());
/// ```
///
/// ## 与专用宏的对比
///
/// ```rust
/// use puniyu_sender::{sender, sender_friend, sender_group, Role, SenderType};
///
/// // 使用统一宏
/// let friend = sender!(Friend, user_id: "123456", nick: "Alice");
/// let group = sender!(Group, user_id: "789012", role: Role::Admin);
/// assert!(matches!(friend, SenderType::Friend(_)));
/// assert!(matches!(group, SenderType::Group(_)));
///
/// // 使用专用宏（返回具体发送者类型）
/// let friend = sender_friend!(user_id: "123456", nick: "Alice");
/// let group = sender_group!(user_id: "789012", role: Role::Admin);
/// ```
///
/// # 注意
///
/// - 此宏只支持命名字段语法，不支持位置参数
/// - 如果需要使用位置参数（如 `sender_friend!("123456")`），请直接使用专用宏
#[macro_export]
macro_rules! sender {
    (Friend, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Friend($crate::sender_friend!( $( $key : $value ),+ ))
    };

    (Group, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Group($crate::sender_group!( $( $key : $value ),+ ))
    };

    (GroupTemp, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::GroupTemp($crate::sender_group_temp!( $( $key : $value ),+ ))
    };

    (Guild, $( $key:ident : $value:expr ),+ $(,)?) => {
        $crate::SenderType::Guild($crate::sender_guild!( $( $key : $value ),+ ))
    };
}
