use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug, Default, Clone, PartialEq, Eq, Display, Deserialize, Serialize, EnumString, IntoStaticStr,
)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdapterConnProtocol {
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
