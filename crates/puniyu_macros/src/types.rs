use syn::{
	Expr, LitBool, LitInt, LitStr, Result, Token, bracketed,
	parse::{Parse, ParseStream},
};

pub(crate) struct ConfigArgs {
	pub name: Option<String>,
}

pub(crate) struct AdapterArgs {
	pub runtime: Expr,
	pub server: Option<Expr>,
}

pub(crate) struct HookArgs {
	pub name: Option<String>,
	pub hook_type: Option<LitStr>,
	pub priority: Option<u32>,
}

pub(crate) struct PluginArg {
	pub desc: Option<String>,
	pub prefix: Option<String>,
	pub server: Option<Expr>,
}

pub(crate) struct TaskArgs {
	pub name: Option<String>,
	pub cron: LitStr,
}

pub(crate) struct ArgType {
	pub name: String,
	pub arg_type: Option<LitStr>,
	pub mode: Option<LitStr>,
	pub required: Option<bool>,
	pub desc: Option<String>,
}

pub(crate) struct CommandArgs {
	pub name: Option<String>,
	pub priority: Option<u32>,
	pub desc: Option<String>,
	pub alias: Option<Vec<String>>,
	pub permission: Option<LitStr>,
}

impl ConfigArgs {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl AdapterArgs {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl HookArgs {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl PluginArg {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl TaskArgs {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl ArgType {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl CommandArgs {
	pub fn parse_tokens(input: proc_macro2::TokenStream) -> Result<Self> {
		syn::parse2(input)
	}
}

impl Parse for ConfigArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut name = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"name" => assign_option(&mut name, key, parser.parse_string()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { name })
	}
}

impl Parse for AdapterArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut runtime = None;
		let mut server = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"runtime" => assign_option(&mut runtime, key, parser.parse_expr()?)?,
				"server" => assign_option(&mut server, key, parser.parse_expr()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { runtime: require_field(runtime, "runtime")?, server })
	}
}

impl Parse for HookArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut name = None;
		let mut hook_type = None;
		let mut priority = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"name" => assign_option(&mut name, key, parser.parse_string()?)?,
				"hook_type" | "type" | "r#type" => {
					assign_option(&mut hook_type, key, parser.parse_lit_str()?)?
				}
				"priority" => assign_option(&mut priority, key, parser.parse_u32()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { name, hook_type, priority })
	}
}

impl Parse for PluginArg {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut desc = None;
		let mut prefix = None;
		let mut server = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"desc" => assign_option(&mut desc, key, parser.parse_string()?)?,
				"prefix" => assign_option(&mut prefix, key, parser.parse_string()?)?,
				"server" => assign_option(&mut server, key, parser.parse_expr()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { desc, prefix, server })
	}
}

impl Parse for TaskArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut name = None;
		let mut cron = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"name" => assign_option(&mut name, key, parser.parse_string()?)?,
				"cron" => assign_option(&mut cron, key, parser.parse_lit_str()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { name, cron: require_field(cron, "cron")? })
	}
}

impl Parse for ArgType {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut name = None;
		let mut arg_type = None;
		let mut mode = None;
		let mut required = None;
		let mut desc = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"name" => assign_option(&mut name, key, parser.parse_string()?)?,
				"arg_type" | "type" | "r#type" => {
					assign_option(&mut arg_type, key, parser.parse_lit_str()?)?
				}
				"mode" => assign_option(&mut mode, key, parser.parse_lit_str()?)?,
				"required" => assign_option(&mut required, key, parser.parse_bool()?)?,
				"desc" => assign_option(&mut desc, key, parser.parse_string()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { name: require_field(name, "name")?, arg_type, mode, required, desc })
	}
}

impl Parse for CommandArgs {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut parser = KeyValueParser::new(input);
		let mut name = None;
		let mut priority = None;
		let mut desc = None;
		let mut alias = None;
		let mut permission = None;

		while let Some(key) = parser.next_key()? {
			match key.as_str() {
				"name" => assign_option(&mut name, key, parser.parse_string()?)?,
				"priority" => assign_option(&mut priority, key, parser.parse_u32()?)?,
				"desc" => assign_option(&mut desc, key, parser.parse_string()?)?,
				"alias" => assign_option(&mut alias, key, parser.parse_string_array()?)?,
				"permission" => assign_option(&mut permission, key, parser.parse_lit_str()?)?,
				_ => return Err(parser.unknown_key(&key)),
			}
		}

		Ok(Self { name, priority, desc, alias, permission })
	}
}

struct KeyValueParser<'a> {
	input: ParseStream<'a>,
	parsed_any: bool,
}

impl<'a> KeyValueParser<'a> {
	fn new(input: ParseStream<'a>) -> Self {
		Self { input, parsed_any: false }
	}

	fn next_key(&mut self) -> Result<Option<String>> {
		if self.input.is_empty() {
			return Ok(None);
		}
		if self.parsed_any {
			self.input.parse::<Token![,]>()?;
			if self.input.is_empty() {
				return Ok(None);
			}
		} else {
			while self.input.peek(Token![,]) {
				self.input.parse::<Token![,]>()?;
			}
			if self.input.is_empty() {
				return Ok(None);
			}
		}
		let ident: syn::Ident = self.input.parse()?;
		self.input.parse::<Token![=]>()?;
		self.parsed_any = true;
		Ok(Some(ident.to_string()))
	}

	fn parse_lit_str(&mut self) -> Result<LitStr> {
		self.input.parse()
	}

	fn parse_string(&mut self) -> Result<String> {
		Ok(self.parse_lit_str()?.value())
	}

	fn parse_bool(&mut self) -> Result<bool> {
		Ok(self.input.parse::<LitBool>()?.value)
	}

	fn parse_u32(&mut self) -> Result<u32> {
		self.input.parse::<LitInt>()?.base10_parse()
	}

	fn parse_expr(&mut self) -> Result<Expr> {
		self.input.parse()
	}

	fn parse_string_array(&mut self) -> Result<Vec<String>> {
		let content;
		bracketed!(content in self.input);
		let mut values = Vec::new();
		while !content.is_empty() {
			values.push(content.parse::<LitStr>()?.value());
			if content.is_empty() {
				break;
			}
			content.parse::<Token![,]>()?;
		}
		Ok(values)
	}

	fn unknown_key(&self, key: &str) -> syn::Error {
		syn::Error::new(self.input.span(), format!("unknown attribute key `{key}`"))
	}
}

fn assign_option<T>(slot: &mut Option<T>, key: String, value: T) -> Result<()> {
	if slot.is_some() {
		return Err(syn::Error::new(
			proc_macro2::Span::call_site(),
			format!("duplicate attribute key `{key}`"),
		));
	}
	*slot = Some(value);
	Ok(())
}

fn require_field<T>(value: Option<T>, name: &str) -> Result<T> {
	value.ok_or_else(|| {
		syn::Error::new(
			proc_macro2::Span::call_site(),
			format!("missing required attribute key `{name}`"),
		)
	})
}
