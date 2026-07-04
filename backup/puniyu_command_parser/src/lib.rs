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
//!     .build()
//!     .parse("@bot !greet --name Alice")?;
//!
//! assert_eq!(parser.command_name(), "greet");
//! assert!(parser.contains("name"));
//! # Ok::<(), puniyu_command_parser::Error>(())
//! ```

mod args;
mod error;
mod input;

#[doc(inline)]
pub use error::Error;

use input::InputParser;
use puniyu_command::{ArgValue, CommandRegistry};
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

    /// 构建命令解析器。
    pub fn build(self) -> CommandParser {
        CommandParser {
            input_parser: InputParser::new().aliases(self.alias).prefixes(self.prefix),
        }
    }
}

/// 命令解析器。
pub struct CommandParser {
    input_parser: InputParser,
}

impl Default for CommandParser {
    fn default() -> Self {
        Self { input_parser: InputParser::new() }
    }
}

impl CommandParser {
    /// 创建命令解析器构建器。
    pub fn builder() -> CommandParserBuilder {
        CommandParserBuilder::new()
    }

    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 解析命令字符串。
    pub fn parse(&self, input: &str) -> Result<ParseResult, Error> {
        let raw = self.input_parser.parse(input)?;

        let mut commands = CommandRegistry::get_with_command_name(&raw.command);
        if commands.is_empty() {
            commands = CommandRegistry::get_with_command_alias(&raw.command);
        }
        if commands.is_empty() {
            return Err(Error::UnknownCommand { name: raw.command });
        }

        let command = commands[0].handle.get();
        let arg_defs = command.args();
        let parsed_args = args::parse_args(&raw.args, &arg_defs)?;

        Ok(ParseResult { command_name: raw.command, parsed_args })
    }
}

/// 命令解析结果。
pub struct ParseResult {
    command_name: String,
    parsed_args: HashMap<String, ArgValue>,
}

impl ParseResult {
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

    /// 返回参数表。
    pub fn into_inner(self) -> HashMap<String, ArgValue> {
        self.parsed_args
    }
}
