use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdapterPlatform {
	/// QQ 平台。
	QQ,
	/// 微信平台。
	Wechat,
	/// Telegram 平台。
	Telegram,
	/// Discord 平台。
	Discord,
	/// Kook 平台。
	Kook,
	/// 其他平台。
	#[default]
	Other,
}