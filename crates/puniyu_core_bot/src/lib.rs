use bytes::Bytes;

pub trait Bot: Send + Sync {
	/// 机器人ID
	fn id(&self) -> &str;
	/// 机器人名称
	fn name(&self) -> &str;
	/// 机器人头像
	fn avatar(&self) -> Bytes;
}