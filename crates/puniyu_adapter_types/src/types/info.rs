use bon::Builder;
use jiff::Timestamp;
use puniyu_semver::Version;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};
use smol_str::SmolStr;

/// 适配器平台。
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

/// 适配器标准接口协议。
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

/// 适配器协议实现。
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

/// 适配器通信方式。
#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdapterCommunication {
	/// HTTP
	Http,
	/// WebSocket 服务端
	WebSocketServer,
	/// WebSocket 客户端
	WebSocketClient,
	/// gRPC
	Grpc,
	/// SSE
	Sse,
	/// 其他通信方式。
	#[default]
	Other,
}

/// 适配器元信息。
#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
#[builder(on(SmolStr, into))]
pub struct AdapterInfo {
	/// 适配器名称
	#[builder(default)]
	pub name: SmolStr,
	/// 适配器作者
	#[builder(default)]
	pub author: Vec<SmolStr>,
	/// 适配器描述
	pub description: Option<SmolStr>,
	/// 适配器版本。
	#[builder(default = AdapterInfo::default_version())]
	pub version: Version,
	/// 适配器平台。
	#[builder(default)]
	pub platform: AdapterPlatform,
	/// 适配器标准。
	#[builder(default)]
	pub standard: AdapterStandard,
	/// 适配器协议实现。
	#[builder(default)]
	pub protocol: AdapterProtocol,
	/// 适配器通信方式。
	#[builder(default)]
	pub communication: AdapterCommunication,
	/// 适配器通信地址
	pub address: Option<String>,
	/// 连接时间。
	#[builder(default = AdapterInfo::default_connect_time())]
	pub connect_time: Timestamp,
	/// 鉴权密钥。
	pub secret: Option<String>,
}

impl PartialEq for AdapterInfo {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.author == other.author
			&& self.version == other.version
			&& self.platform == other.platform
			&& self.standard == other.standard
			&& self.protocol == other.protocol
			&& self.communication == other.communication
			&& self.address == other.address
			&& self.secret == other.secret
			&& self.connect_time == other.connect_time
	}
}

impl Eq for AdapterInfo {}

impl AdapterInfo {
	const fn default_version() -> Version {
		Version::new(0, 1, 0)
	}

	fn default_connect_time() -> Timestamp {
		Timestamp::now()
	}
}

/// 创建 `AdapterInfo` 的便捷宏。
#[macro_export]
macro_rules! adapter_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::AdapterInfo::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($name:expr, $platform:expr, $protocol:expr) => {{
		adapter_info!(
			name: $name,
			platform: $platform,
			protocol: $protocol,
		)
	}};
}
