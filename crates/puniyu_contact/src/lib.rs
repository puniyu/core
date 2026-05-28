//! # puniyu_contact
//!
//! 统一的联系人类型，覆盖好友、群聊、群临时与频道场景。
//!
//! ## 特性
//!
//! - 提供 `FriendContact`、`GroupContact`、`GroupTempContact` 与 `GuildContact`
//! - 提供统一接口 `Contact`
//! - 提供统一枚举 `ContactType`
//! - 提供构建宏 `contact!`、`contact_friend!`、`contact_group!`、`contact_group_temp!` 与 `contact_guild!`
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_contact::{contact, Contact};
//!
//! let friend = contact!(Friend, peer: "123456", name: "Alice");
//! let group = contact!(Group, peer: "789012", name: "Dev Team");
//! let group_temp = contact!(GroupTemp, peer: "789012", name: "Temp Team");
//! let guild = contact!(Guild, peer: "123", name: "Guild Channel", sub_name: "General");
//!
//! assert!(friend.is_friend());
//! assert_eq!(friend.peer(), "123456");
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
use std::borrow::Cow;
use std::fmt::Debug;
use strum::{Display, IntoStaticStr};

/// 联系人类型枚举
///
/// 统一的联系人类型，可以是好友或群聊。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{ContactType, FriendContact, GroupContact, GroupTempContact};
///
/// // 创建好友联系人
/// let friend = FriendContact::new("123456", Some("Alice"));
/// let contact = ContactType::Friend(friend);
///
/// // 创建群聊联系人
/// let group = GroupContact::new("789012", Some("Dev Team"));
/// let contact = ContactType::Group(group);
///
/// // 创建群临时联系人
/// let group_temp = GroupTempContact::new("246810", Some("Temp Team"));
/// let contact = ContactType::GroupTemp(group_temp);
/// ```
#[derive(Debug, Clone, PartialEq, Display, IntoStaticStr, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'c"))]
#[strum(serialize_all = "lowercase")]
pub enum ContactType<'c> {
	/// 好友联系人
	Friend(FriendContact<'c>),
	/// 群聊联系人
	Group(GroupContact<'c>),
	/// 群临时联系人
	GroupTemp(GroupTempContact<'c>),
	/// 频道联系人
	Guild(GuildContact<'c>),
}

impl<'c> ContactType<'c> {
	/// 创建联系人类型
	///
	/// 根据场景类型自动创建对应的联系人。
	///
	/// # 参数
	///
	/// * `scene` - 场景类型（好友或群聊）
	/// * `peer` - 联系人 ID
	/// * `name` - 联系人名称（可选）
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_contact::{ContactType, SceneType};
	///
	/// // 创建好友联系人
	/// let friend = ContactType::new(SceneType::Friend, "123456", Some("Alice"));
	///
	/// // 创建群聊联系人
	/// let group = ContactType::new(SceneType::Group, "789012", Some("Dev Team"));
	/// ```
	pub fn new<P, N>(scene: SceneType, peer: P, name: Option<N>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		let peer = peer.into();
		let name = name.map(Into::into);

		match scene {
			SceneType::Friend => match name {
				Some(name) => {
					ContactType::Friend(FriendContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Friend(FriendContact::builder().peer(peer).build()),
			},
			SceneType::Group => match name {
				Some(name) => {
					ContactType::Group(GroupContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Group(GroupContact::builder().peer(peer).build()),
			},
			SceneType::GroupTemp => match name {
				Some(name) => ContactType::GroupTemp(
					GroupTempContact::builder().peer(peer).name(name).build(),
				),
				None => ContactType::GroupTemp(GroupTempContact::builder().peer(peer).build()),
			},
			SceneType::Guild => match name {
				Some(name) => {
					ContactType::Guild(GuildContact::builder().peer(peer).name(name).build())
				}
				None => ContactType::Guild(GuildContact::builder().peer(peer).build()),
			},
		}
	}

	/// 尝试获取好友联系人的引用
	///
	/// 如果是好友联系人则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_friend(&self) -> Option<&FriendContact<'c>> {
		match self {
			ContactType::Friend(f) => Some(f),
			_ => None,
		}
	}

	/// 尝试获取群聊联系人的引用
	///
	/// 如果是群聊联系人则返回 [`Some`]，否则返回 [`None`]。
	pub fn as_group(&self) -> Option<&GroupContact<'c>> {
		match self {
			ContactType::Group(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取群临时联系人的引用
	pub fn as_group_temp(&self) -> Option<&GroupTempContact<'c>> {
		match self {
			ContactType::GroupTemp(g) => Some(g),
			_ => None,
		}
	}

	/// 尝试获取频道联系人的引用
	pub fn as_guild(&self) -> Option<&GuildContact<'c>> {
		match self {
			ContactType::Guild(g) => Some(g),
			_ => None,
		}
	}

	pub fn is_friend(&self) -> bool {
		matches!(self, ContactType::Friend(_))
	}

	pub fn is_group(&self) -> bool {
		matches!(self, ContactType::Group(_))
	}

	pub fn is_group_temp(&self) -> bool {
		matches!(self, ContactType::GroupTemp(_))
	}

	pub fn is_guild(&self) -> bool {
		matches!(self, ContactType::Guild(_))
	}
}

impl<'c> Contact for ContactType<'c> {
	fn scene(&self) -> &SceneType {
		match self {
			ContactType::Friend(contact) => contact.scene(),
			ContactType::Group(contact) => contact.scene(),
			ContactType::GroupTemp(contact) => contact.scene(),
			ContactType::Guild(contact) => contact.scene(),
		}
	}

	fn peer(&self) -> &str {
		match self {
			ContactType::Friend(contact) => contact.peer(),
			ContactType::Group(contact) => contact.peer(),
			ContactType::GroupTemp(contact) => contact.peer(),
			ContactType::Guild(contact) => contact.peer(),
		}
	}

	fn name(&self) -> Option<&str> {
		match self {
			ContactType::Friend(contact) => contact.name(),
			ContactType::Group(contact) => contact.name(),
			ContactType::GroupTemp(contact) => contact.name(),
			ContactType::Guild(contact) => contact.name(),
		}
	}
}

impl<'c> From<FriendContact<'c>> for ContactType<'c> {
	fn from(value: FriendContact<'c>) -> Self {
		Self::Friend(value)
	}
}

impl<'c> From<GroupContact<'c>> for ContactType<'c> {
	fn from(value: GroupContact<'c>) -> Self {
		Self::Group(value)
	}
}

impl<'c> From<GroupTempContact<'c>> for ContactType<'c> {
	fn from(value: GroupTempContact<'c>) -> Self {
		Self::GroupTemp(value)
	}
}

impl<'c> From<GuildContact<'c>> for ContactType<'c> {
	fn from(value: GuildContact<'c>) -> Self {
		Self::Guild(value)
	}
}

impl<'c> From<&'c FriendContact<'c>> for ContactType<'c> {
	fn from(value: &'c FriendContact<'c>) -> Self {
		Self::Friend(FriendContact::new(value.peer(), value.name()))
	}
}

impl<'c> From<&'c GroupContact<'c>> for ContactType<'c> {
	fn from(value: &'c GroupContact<'c>) -> Self {
		Self::Group(GroupContact::new(value.peer(), value.name()))
	}
}

impl<'c> From<&'c GroupTempContact<'c>> for ContactType<'c> {
	fn from(value: &'c GroupTempContact<'c>) -> Self {
		Self::GroupTemp(GroupTempContact::new(value.peer(), value.name()))
	}
}

impl<'c> From<&'c GuildContact<'c>> for ContactType<'c> {
	fn from(value: &'c GuildContact<'c>) -> Self {
		Self::Guild(GuildContact::new(value.peer(), value.name(), value.sub_name()))
	}
}

#[macro_export]
macro_rules! contact {
	(Friend, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Friend($crate::contact_friend!($( $key : $value ),+))
	};
	(Group, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Group($crate::contact_group!($( $key : $value ),+))
	};
	(GroupTemp, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::GroupTemp($crate::contact_group_temp!($( $key : $value ),+))
	};
	(Guild, $( $key:ident : $value:expr ),+ $(,)? ) => {
		$crate::ContactType::Guild($crate::contact_guild!($( $key : $value ),+))
	};
}
