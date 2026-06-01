//! # puniyu_command_parser
//!
//! 统一的 puniyu 命令解析器，覆盖命令文本解析、别名剥离、前缀处理与参数验证场景。
//!
//! ## 特性
//!
//! - 提供 [`CommandParser`] 与 [`CommandParserBuilder`]
//! - 支持按顺序剥离 bot 别名和命令前缀
//! - 支持结合 `puniyu_command` 注册表做参数验证
//! - 支持字符串、整数、浮点数和布尔参数
//!
//! ## 示例
//!
//! ```rust,no_run
//! use puniyu_command_parser::CommandParser;
//!
//! let parser = CommandParser::builder()
//!     .aliases(vec!["@bot".to_string()])
//!     .prefix(vec!["!".to_string()])
//!     .parse("@bot !greet --name Alice")?;
//!
//! assert_eq!(parser.command_name(), "greet");
//! assert!(parser.contains("name"));
//! # Ok::<(), puniyu_command_parser::Error>(())
//! ```

mod error;
#[doc(inline)]
pub use error::Error;

use clap::builder::ValueParser;
use puniyu_command::{CommandRegistry, Arg, ArgMode, ArgType, ArgValue};
use std::collections::HashMap;

/// 命令解析器构建器。
#[derive(Debug, Clone, Default)]
pub struct CommandParserBuilder {
	alias: Vec<String>,
	prefix: Vec<String>,
}

impl CommandParserBuilder {
	/// 创建构建器。
	pub fn new() -> Self {
		Self::default()
	}

	/// 设置 bot 别名列表。
	pub fn aliases<I, S>(mut self, aliases: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.alias = aliases.into_iter().map(Into::into).collect();
		self
	}

	/// 设置命令前缀列表。
	pub fn prefix<I, S>(mut self, prefix: I) -> Self
	where
		I: IntoIterator<Item = S>,
		S: Into<String>,
	{
		self.prefix = prefix.into_iter().map(Into::into).collect();
		self
	}

	/// 解析命令字符串。
	pub fn parse(self, input: &str) -> Result<CommandParser, Error> {
		let options = ParseOptions { alias: self.alias, prefixes: self.prefix };
		CommandParser::parse_with_options(input, &options)
	}
}

/// 命令解析器。
pub struct CommandParser {
	command_name: String,
	parsed_args: HashMap<String, ArgValue>,
}

/// 解析选项。
#[derive(Debug, Clone, Default)]
struct ParseOptions {
	alias: Vec<String>,
	prefixes: Vec<String>,
}

impl CommandParser {
	/// 创建命令解析器构建器。
	pub fn builder() -> CommandParserBuilder {
		CommandParserBuilder::new()
	}

	/// 从字符串解析命令。
	pub fn parse(input: &str) -> Result<Self, Error> {
		Self::new(input)
	}

	/// 从字符串解析命令，并自动查询注册表验证参数。
	pub fn new(input: &str) -> Result<Self, Error> {
		Self::parse_with_options(input, &ParseOptions::default())
	}

	fn parse_with_options(input: &str, options: &ParseOptions) -> Result<Self, Error> {
		let text =
			Self::strip_prefix(Self::strip_bot_alias(input, &options.alias), &options.prefixes);
		let text = text.trim();
		let args = shlex::split(text).ok_or(Error::EmptyInput)?;
		let command_name = args.first().ok_or(Error::EmptyInput)?.clone();

		let mut commands = CommandRegistry::get_with_command_name(&command_name);
		if commands.is_empty() {
			commands = CommandRegistry::get_with_command_alias(&command_name);
		}
		if commands.is_empty() {
			return Err(Error::UnknownArgument { arg_name: command_name });
		}

		let command_info = &commands[0];
		let arg_defs = command_info.builder.args();
		let aliases = command_info.builder.alias();

		let cmd = Self::build_command(&command_name, &aliases, &arg_defs);
		let matches =
			cmd.try_get_matches_from(&args[1..]).map_err(|e| Self::convert_error(e, &arg_defs))?;

		let mut parsed_args = HashMap::new();
		for arg_def in &arg_defs {
			if let Some(value) = Self::extract_value(&matches, arg_def) {
				parsed_args.insert(arg_def.name.to_string(), value);
			}
		}

		Ok(Self { command_name, parsed_args })
	}

	fn strip_bot_alias<'t>(text: &'t str, aliases: &[String]) -> &'t str {
		Self::strip_pattern(text, aliases)
	}

	fn strip_prefix<'t>(text: &'t str, prefixes: &[String]) -> &'t str {
		Self::strip_pattern(text, prefixes)
	}

	fn strip_pattern<'t>(text: &'t str, patterns: &[String]) -> &'t str {
		patterns
			.iter()
			.find_map(|pattern| {
				(!pattern.is_empty()).then(|| text.strip_prefix(pattern.as_str())).flatten()
			})
			.map(str::trim_start)
			.unwrap_or(text)
	}

	/// 获取命令名称。
	pub fn command_name(&self) -> &str {
		&self.command_name
	}

	/// 获取原始参数值。
	pub fn get(&self, name: &str) -> Option<&ArgValue> {
		self.parsed_args.get(name)
	}

	/// 检查参数是否存在。
	pub fn contains(&self, name: &str) -> bool {
		self.parsed_args.contains_key(name)
	}

	/// 获取所有参数名。
	pub fn keys(&self) -> impl Iterator<Item = &String> {
		self.parsed_args.keys()
	}

	/// 获取参数数量。
	pub fn len(&self) -> usize {
		self.parsed_args.len()
	}

	/// 检查是否为空。
	pub fn is_empty(&self) -> bool {
		self.parsed_args.is_empty()
	}

	/// 消耗 `self` 并返回参数表。
	pub fn into_inner(self) -> HashMap<String, ArgValue> {
		self.parsed_args
	}

	fn build_command(command_name: &str, aliases: &[&str], arg_defs: &[Arg]) -> clap::Command {
		let mut cmd = clap::Command::new(command_name.to_string())
			.no_binary_name(true)
			.disable_help_flag(true)
			.disable_version_flag(true);
		if !aliases.is_empty() {
			let alias: Vec<String> = aliases.iter().map(|s| s.to_string()).collect();
			cmd = cmd.aliases(alias);
		}

		for arg in arg_defs {
			cmd = cmd.arg(Self::build_clap_arg(arg));
		}
		cmd
	}

	fn build_clap_arg(arg: &Arg) -> clap::Arg {
		let mut clap_arg = clap::Arg::new(arg.name.to_string());

		clap_arg = match arg.arg_type {
			ArgType::String => clap_arg.value_parser(ValueParser::string()),
			ArgType::Int => clap_arg.value_parser(clap::value_parser!(i64)),
			ArgType::Float => clap_arg.value_parser(clap::value_parser!(f64)),
			ArgType::Bool => clap_arg
				.value_parser(clap::value_parser!(bool))
				.num_args(0..=1)
				.default_missing_value("true"),
		};

		match arg.mode {
			ArgMode::Positional => clap_arg.required(arg.required),
			ArgMode::Named => clap_arg.long(arg.name.to_string()).required(arg.required),
		}
	}

	fn extract_value(matches: &clap::ArgMatches, arg: &Arg) -> Option<ArgValue> {
		match arg.arg_type {
			ArgType::String => {
				matches.get_one::<String>(&arg.name).map(|s| ArgValue::String(s.to_string()))
			}
			ArgType::Int => matches.get_one::<i64>(&arg.name).copied().map(ArgValue::Int),
			ArgType::Float => matches.get_one::<f64>(&arg.name).copied().map(ArgValue::Float),
			ArgType::Bool => matches.get_one::<bool>(&arg.name).copied().map(ArgValue::Bool),
		}
	}

	fn convert_error(e: clap::Error, arg_defs: &[Arg]) -> Error {
		use clap::error::{ContextKind, ErrorKind};

		let arg_name = e
			.get(ContextKind::InvalidArg)
			.map(|v| v.to_string())
			.unwrap_or_default()
			.trim_matches(|c| c == '<' || c == '>' || c == '-')
			.to_string();

		let expected_type = arg_defs
			.iter()
			.find(|a| a.name == arg_name)
			.map(|a| match a.arg_type {
				ArgType::String => "string",
				ArgType::Int => "integer",
				ArgType::Float => "float",
				ArgType::Bool => "boolean",
			})
			.unwrap_or("valid value")
			.to_string();

		match e.kind() {
			ErrorKind::InvalidValue | ErrorKind::ValueValidation => {
				Error::InvalidValue { arg_name, expected_type }
			}
			ErrorKind::UnknownArgument => Error::UnknownArgument { arg_name },
			ErrorKind::TooManyValues => Error::TooManyValues { arg_name },
			ErrorKind::TooFewValues => Error::TooFewValues { arg_name },
			ErrorKind::MissingRequiredArgument => Error::MissingRequired { arg_name },
			_ => Error::EmptyInput,
		}
	}
}
