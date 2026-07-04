//! 配置相关错误类型。

use std::io;
use thiserror::Error;

/// 配置错误。
#[derive(Error, Debug)]
pub enum Error {
	/// 文件读写错误。
	#[error("IO error: {0}")]
	Io(#[from] io::Error),

	/// TOML 反序列化错误。
	#[error("TOML parse error: {0}")]
	Deserialize(#[from] toml::de::Error),

	/// TOML 序列化错误。
	#[error("TOML serialize error: {0}")]
	Serialize(#[from] toml::ser::Error),
}
