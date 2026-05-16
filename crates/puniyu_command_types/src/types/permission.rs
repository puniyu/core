//! 命令权限类型。

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::{Display, EnumString, IntoStaticStr};

/// 命令权限级别。
#[repr(u8)]
#[derive(
	Debug,
	Default,
	Copy,
	Clone,
	PartialEq,
	PartialOrd,
	EnumString,
	Display,
	IntoStaticStr,
	Serialize_repr,
	Deserialize_repr,
)]
#[strum(serialize_all = "lowercase")]
pub enum Permission {
	/// 所有人都可以执行。
	#[default]
	All = 0,
	/// 仅管理员可以执行。
	Admin = 1,
	/// 仅群主/频道主可以执行。
	Owner = 2,
	/// 仅主人可以执行。
	Master = 3,
}

impl Permission {
	/// 判断当前权限是否满足目标权限。
	pub const fn satisfies(self, required: Permission) -> bool {
		matches!(required, Permission::All) || (self as u8) >= (required as u8)
	}
}
