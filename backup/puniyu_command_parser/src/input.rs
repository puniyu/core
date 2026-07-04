use crate::error::Error;


pub(crate) struct RawInput {
    /// 命令名称。
    pub command: String,
    /// 命令名之后的 token 列表。
    pub args: Vec<String>,
}

pub(crate) struct InputParser {
    alias: Vec<String>,
    prefixes: Vec<String>,
}

impl InputParser {
    pub fn new() -> Self {
        Self { alias: Vec::new(), prefixes: Vec::new() }
    }

    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.alias = aliases;
        self
    }

    pub fn prefixes(mut self, prefixes: Vec<String>) -> Self {
        self.prefixes = prefixes;
        self
    }

    pub fn parse(&self, input: &str) -> Result<RawInput, Error> {
        let text = Self::strip_pattern(Self::strip_pattern(input, &self.alias), &self.prefixes);
        let text = text.trim();
        let tokens = shlex::split(text).ok_or(Error::EmptyInput)?;
        if tokens.is_empty() || tokens[0].is_empty() {
            return Err(Error::EmptyInput);
        }
        let command = tokens[0].clone();
        let args = tokens[1..].to_vec();
        Ok(RawInput { command, args })
    }

    fn strip_pattern<'t>(text: &'t str, patterns: &[String]) -> &'t str {
        patterns
            .iter()
            .find_map(|p| {
                if p.is_empty() {
                    None
                } else {
                    text.strip_prefix(p.as_str())
                }
            })
            .map(str::trim_start)
            .unwrap_or(text)
    }
}
