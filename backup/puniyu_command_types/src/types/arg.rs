//! 命令参数类型。

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use strum::{Display, EnumString, IntoStaticStr};

/// 命令参数数据类型。
#[derive(
	Debug, Copy, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ArgType {
	/// 字符串参数。
	String,
	/// 整数参数。
	Int,
	/// 浮点数参数。
	Float,
	/// 布尔参数。
	Bool,
}

/// 命令参数匹配模式。
#[derive(
	Debug,
	Default,
	Copy,
	Clone,
	PartialEq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ArgMode {
	/// 位置参数。
	#[default]
	Positional,
	/// 命名参数。
	Named,
}

/// 命令参数定义。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Arg<'a> {
	/// 参数名。
	pub name: Cow<'a, str>,
	/// 参数类型。
	pub arg_type: ArgType,
	/// 参数模式。
	pub mode: ArgMode,
	/// 是否必需。
	pub required: bool,
	/// 参数描述。
	pub description: Option<Cow<'a, str>>,
}

impl<'a> Default for Arg<'a> {
	fn default() -> Self {
		Self {
			name: Cow::Borrowed(""),
			arg_type: ArgType::String,
			mode: ArgMode::Positional,
			required: false,
			description: None,
		}
	}
}

impl<'a> Arg<'a> {
	/// 创建参数定义。
	pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
		Self { name: name.into(), ..Default::default() }
	}

	/// 创建字符串参数。
	pub fn string(name: impl Into<Cow<'a, str>>) -> Self {
		Self::new(name).with_type(ArgType::String)
	}

	/// 创建整数参数。
	pub fn int(name: impl Into<Cow<'a, str>>) -> Self {
		Self::new(name).with_type(ArgType::Int)
	}

	/// 创建浮点数参数。
	pub fn float(name: impl Into<Cow<'a, str>>) -> Self {
		Self::new(name).with_type(ArgType::Float)
	}

	/// 创建布尔参数。
	pub fn bool(name: impl Into<Cow<'a, str>>) -> Self {
		Self::new(name).with_type(ArgType::Bool)
	}

	/// 设置参数类型。
	pub fn with_type(mut self, arg_type: ArgType) -> Self {
		self.arg_type = arg_type;
		self
	}

	/// 标记为必需参数。
	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	/// 标记为可选参数。
	pub fn optional(mut self) -> Self {
		self.required = false;
		self
	}

	/// 设置参数描述。
	pub fn description(mut self, desc: impl Into<Cow<'a, str>>) -> Self {
		self.description = Some(desc.into());
		self
	}

	/// 设置为位置参数。
	pub fn positional(mut self) -> Self {
		self.mode = ArgMode::Positional;
		self
	}

	/// 设置为命名参数。
	pub fn named(mut self) -> Self {
		self.mode = ArgMode::Named;
		self
	}
}

/// 命令参数值。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ArgValue {
	/// 字符串值。
	String(String),
	/// 整数值。
	Int(i64),
	/// 浮点数值。
	Float(f64),
	/// 布尔值。
	Bool(bool),
}

impl ArgValue {
	/// 获取字符串值。
	pub fn as_str(&self) -> Option<&str> {
		match self {
			ArgValue::String(s) => Some(s),
			_ => None,
		}
	}

	/// 获取整数值。
	pub fn as_int(&self) -> Option<i64> {
		match self {
			ArgValue::Int(i) => Some(*i),
			_ => None,
		}
	}

	/// 获取浮点数值。
	pub fn as_float(&self) -> Option<f64> {
		match self {
			ArgValue::Float(f) => Some(*f),
			_ => None,
		}
	}

	/// 获取布尔值。
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			ArgValue::Bool(b) => Some(*b),
			_ => None,
		}
	}
}

impl fmt::Display for ArgValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ArgValue::String(s) => write!(f, "{}", s),
			ArgValue::Int(i) => write!(f, "{}", i),
			ArgValue::Float(fl) => write!(f, "{}", fl),
			ArgValue::Bool(b) => write!(f, "{}", b),
		}
	}
}

impl From<String> for ArgValue {
	fn from(s: String) -> Self {
		ArgValue::String(s)
	}
}

impl From<&str> for ArgValue {
	fn from(s: &str) -> Self {
		ArgValue::String(s.to_string())
	}
}

impl From<i64> for ArgValue {
	fn from(i: i64) -> Self {
		Self::Int(i)
	}
}

impl From<u64> for ArgValue {
	fn from(u: u64) -> Self {
		Self::Int(u as i64)
	}
}
impl From<f64> for ArgValue {
	fn from(f: f64) -> Self {
		Self::Float(f)
	}
}

impl From<bool> for ArgValue {
	fn from(b: bool) -> Self {
		Self::Bool(b)
	}
}
