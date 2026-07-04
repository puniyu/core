use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdapterStandard {
	/// OneBot v11。
	OneBotV11,
	/// OneBot v12。
	OneBotV12,
	/// OICQ。
	Oicq,
	/// ICQQ。
	Icqq,
	/// Milky。
	Milky,
	/// Satori。
	Satori,
	/// 其他标准。
	#[default]
	Other,
}