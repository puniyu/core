use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 群聊角色枚举。
#[derive(
	Debug,
	Default,
	Clone,
	Copy,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Role {
	/// 群主/频道主
	Owner,
	/// 管理员
	Admin,
	/// 群成员、频道成员
	Member,
	/// 未知角色
	#[default]
	Unknown,
}

impl Role {
	/// 是否为群主。
	pub fn is_owner(&self) -> bool {
		matches!(self, Self::Owner)
	}

	/// 是否为管理员。
	pub fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}

	/// 是否为普通成员。
	pub fn is_member(&self) -> bool {
		matches!(self, Self::Member)
	}

	/// 是否为未知角色。
	pub fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown)
	}
}
