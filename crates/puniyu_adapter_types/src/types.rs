mod connection;
pub use connection::*;
mod platform;
use jiff::Timestamp;
pub use platform::*;
mod protocol;
pub use protocol::*;
mod standard;
use puniyu_semver::Version;
pub use standard::*;

use bon::Builder;


use std::time::Duration;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

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
	pub conn_protocol: AdapterConnProtocol,
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
			&& self.conn_protocol == other.conn_protocol
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



#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendMsgType {
	/// 消息 ID。
	pub message_id: String,
	/// 发送时间戳，单位为秒。
	pub time: Duration,
}