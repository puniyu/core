use thiserror::Error;

/// 注册表错误。
#[derive(Error, Debug)]
pub enum Error {
	/// 未找到指定项。
	#[error("not found: {0}")]
	NotFound(String),

	/// 项已存在。
	#[error("exists: {0}")]
	Exists(String),
}
