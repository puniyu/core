use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdapterProtocol {
	/// QQ 官方机器人协议。
	QQBot,
	/// ICQQ。
	Icqq,
	/// OICQ。
	Oicq,
	/// go-cqhttp。
	GoCqHttp,
	/// NapCat。
	NapCat,
	/// LLOneBot。
	LLOneBot,
	/// Conwechat。
	Conwechat,
	/// Lagrange。
	Lagrange,
	/// Yogurt。
	Yogurt,
	/// 控制台协议。
	Console,
	/// 其他协议。
	#[default]
	Other,
}