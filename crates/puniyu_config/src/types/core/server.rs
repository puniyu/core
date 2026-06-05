use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

/// 服务器配置结构
///
/// 定义 HTTP 服务器的监听地址和端口。
///
/// # 示例
///
/// ```toml
/// [server]
/// host = "127.0.0.1"
/// port = 33720
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
	/// 服务器主机地址
	///
	/// 服务器监听的 IP 地址，默认为 127.0.0.1（本地回环）
	#[serde(default = "default_server_host")]
	host: IpAddr,

	/// 服务器端口号
	///
	/// 服务器监听的端口，默认为 33720
	#[serde(default = "default_server_port")]
	port: u16,
}

const fn default_server_host() -> IpAddr {
	let addr = Ipv4Addr::new(127, 0, 0, 1);
	IpAddr::V4(addr)
}
const fn default_server_port() -> u16 {
	10721
}

impl Default for ServerConfig {
	#[inline]
	fn default() -> Self {
		Self { host: default_server_host(), port: default_server_port() }
	}
}

impl ServerConfig {
	/// 获取服务器监听地址。
	pub fn host(&self) -> IpAddr {
		self.host
	}

	/// 获取服务器监听端口。
	pub fn port(&self) -> u16 {
		self.port
	}
}
