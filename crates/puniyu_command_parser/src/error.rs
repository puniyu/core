use thiserror::Error;

/// 命令解析错误。
#[derive(Debug, Clone, Error)]
pub enum Error {
	/// 输入为空。
	#[error("Empty input")]
	EmptyInput,
	/// 参数值无效。
	#[error("Invalid value for argument {arg_name}, expected {expected_type}")]
	InvalidValue { arg_name: String, expected_type: String },
	/// 未知参数。
	#[error("Unknown argument: {arg_name}")]
	UnknownArgument { arg_name: String },
	/// 参数值过多。
	#[error("Too many values provided for argument {arg_name}")]
	TooManyValues { arg_name: String },
	/// 参数值不足。
	#[error("Too few values provided for argument {arg_name}")]
	TooFewValues { arg_name: String },
	/// 缺少必需参数。
	#[error("Missing required argument: {arg_name}")]
	MissingRequired { arg_name: String },
}
