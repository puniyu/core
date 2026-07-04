use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 性别枚举。
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
pub enum Sex {
	/// 男性
	Male,
	/// 女性
	Female,
	/// 未知性别
	#[default]
	Unknown,
}

impl Sex {
	/// 是否为男性。
	pub fn is_male(&self) -> bool {
		matches!(self, Sex::Male)
	}

	/// 是否为女性。
	pub fn is_female(&self) -> bool {
		matches!(self, Sex::Female)
	}

	/// 是否为未知性别。
	pub fn is_unknown(&self) -> bool {
		matches!(self, Sex::Unknown)
	}
}
