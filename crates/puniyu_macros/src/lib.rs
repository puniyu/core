//! # puniyu_macros
//!
//! `puniyu_macros` 提供 puniyu 生态中的过程宏，用于声明插件、适配器、命令、任务、Hook 和配置，减少 trait 实现与 `inventory` 注册样板代码。
//!
//! ## 插件侧宏
//!
//! - [`plugin`]：声明插件入口函数
//! - [`plugin_config`]：声明插件配置结构体
//! - [`plugin_hook`]：声明插件钩子函数
//! - [`command`]：声明命令处理函数
//! - [`arg`]：为命令补充参数描述
//! - [`task`]：声明定时任务函数
//!
//! ## 适配器侧宏
//!
//! - [`adapter`]：声明适配器入口函数
//! - [`adapter_config`]：声明适配器配置结构体
//! - [`adapter_hook`]：声明适配器钩子函数
//!
//! ## 编译期校验
//!
//! 大多数函数类宏会在编译期校验以下内容：
//!
//! - 被标注项是否为 `async fn`
//! - 函数参数和返回类型是否满足约束
//! - 属性参数是否使用合法的 key-value 形式
//! - `cron`、`hook_type`、`permission` 等枚举值或格式是否有效
//!
//! ## 生成行为
//!
//! 这些宏会生成对应 trait 的实现，并通过 `inventory` 注册元数据，供运行时收集插件、适配器、命令、任务、Hook 和配置。
//!
//! ## 示例
//!
//! ```rust, ignore
//! use puniyu_macros::plugin;
//!
//! #[plugin]
//! async fn __main() {}
//! ```

mod adapter;
mod common;
mod plugin;
mod types;
pub(crate) use types::*;

fn parse_attr<T: syn::parse::Parse>(
	tokens: proc_macro::TokenStream,
) -> Result<T, proc_macro::TokenStream> {
	syn::parse(tokens).map_err(|err| err.to_compile_error().into())
}

fn arg_marker(args: proc_macro::TokenStream) -> proc_macro2::TokenStream {
	let args: proc_macro2::TokenStream = args.into();
	quote::quote!(#[__puniyu_arg(#args)])
}

/// 声明适配器配置结构体。
///
/// # 参数
///
/// - `name = "..."`：可选，配置名；默认从结构体名推导并转换为 snake_case。
///
/// # 约束
///
/// - 只能标注在结构体上。
/// - 被标注结构体需要实现 `Default`。
/// - 被标注结构体需要能序列化为 TOML。
///
#[proc_macro_attribute]
pub fn adapter_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemStruct>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match ConfigArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	adapter::config(item, cfg).into()
}

/// 声明适配器 Hook 函数。
///
/// # 参数
///
/// - `name = "..."`：可选，Hook 名；默认从函数名推导。
/// - `hook_type = "..."`：可选，也可写作 `type` 或 `r#type`；默认使用 `HookType::default()`。
/// - `priority = 100`：可选，优先级；默认 `500`。
///
/// `hook_type` 支持：`event`、`event.message`、`event.extension`、`event.all`、`status`、`status.start`、`status.stop`。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 函数必须接收一个 `&HookType` 参数。
/// - 函数必须返回 `puniyu_adapter::Result`。
///
#[proc_macro_attribute]
pub fn adapter_hook(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match HookArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	adapter::hook(item, cfg).into()
}

/// 声明适配器入口函数。
///
/// # 参数
///
/// - `runtime = path`：必填，返回 `Arc<dyn puniyu_adapter::runtime::AdapterRuntime>` 的函数或表达式。
/// - `server = path`：可选，适配器服务函数。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 函数必须返回 `puniyu_adapter::Result`。
/// - 空函数体不会生成 `init` 调用；非空函数体会作为适配器初始化逻辑调用。
///
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::adapter;
///
/// #[adapter(runtime = runtime)]
/// async fn __main() -> puniyu_adapter::Result {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn adapter(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match AdapterArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	adapter::adapter(item, cfg).into()
}

/// 声明插件入口函数。
///
/// # 参数
///
/// - `desc = "..."`：可选，插件描述。
/// - `prefix = "..."`：可选，命令前缀。
/// - `server = path`：可选，插件服务函数。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 当函数体非空时，函数必须返回 `puniyu_plugin::Result`。
/// - 空函数体不会生成 `init` 调用；非空函数体会作为插件初始化逻辑调用。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin;
///
/// #[plugin]
/// async fn __main() {}
/// ```
#[proc_macro_attribute]
pub fn plugin(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match PluginArg::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::plugin(item, cfg).into()
}

/// 声明插件配置结构体。
///
/// # 参数
///
/// - `name = "..."`：可选，配置名；默认从结构体名推导并转换为 snake_case。
///
/// # 约束
///
/// - 只能标注在结构体上。
/// - 被标注结构体需要实现 `Default`。
/// - 被标注结构体需要能序列化为 TOML。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin_config;
///
/// #[derive(Default, serde::Serialize, serde::Deserialize)]
/// #[plugin_config(name = "sample")]
/// struct SampleConfig {
///     value: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin_config(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemStruct>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match ConfigArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::config(item, cfg).into()
}

/// 声明插件命令处理函数。
///
/// # 参数
///
/// - `name = "..."`：可选，命令名；默认从函数名推导并转换为 snake_case。
/// - `priority = 100`：可选，优先级；默认 `500`。
/// - `desc = "..."`：可选，命令描述。
/// - `alias = ["a", "b"]`：可选，命令别名列表。
/// - `permission = "all"`：可选，命令权限；默认 `all`。
///
/// `permission` 支持：`all`、`master`、`owner`、`admin`。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 函数必须接收一个 `&MessageContext` 参数。
/// - 函数必须返回 `puniyu_plugin::Result<CommandAction>`。
/// - 可配合多个 [`arg`] 属性声明命令参数。
///
///
/// # 常见错误
///
/// - 函数不是 `async fn`。
/// - `permission` 值不合法。
/// - 参数或返回类型不匹配。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_context::MessageContext;
/// use puniyu_macros::{arg, command};
/// use puniyu_plugin::command::CommandAction;
///
/// #[command(desc = "echo", permission = "all")]
/// #[arg(name = "message", desc = "消息")]
/// async fn echo(_ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
///     CommandAction::done()
/// }
/// ```
#[proc_macro_attribute]
pub fn command(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match CommandArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::command(item, cfg).into()
}

/// 为 [`command`] 声明命令参数。
///
/// # 参数
///
/// - `name = "..."`：必填，参数名。
/// - `arg_type = "string"`：可选，也可写作 `type` 或 `r#type`；默认 `string`。
/// - `mode = "positional"`：可选，默认 `positional`。
/// - `required = true`：可选，是否必填；默认 `false`。
/// - `desc = "..."`：可选，参数描述。
///
/// `arg_type` 支持：`string`、`integer`、`int`、`float`、`boolean`、`bool`。
/// `mode` 支持：`positional`、`named`。
///
/// # 约束
///
/// - 该属性应与 [`command`] 一起使用。
/// - 该宏会先校验自身参数格式，再写入内部参数标记供 [`command`] 收集处理。
///
#[proc_macro_attribute]
pub fn arg(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	if let Err(err) = ArgType::parse_tokens(args.clone().into()) {
		return err.to_compile_error().into();
	}
	let marker = arg_marker(args);
	let item: proc_macro2::TokenStream = item.into();
	quote::quote!(#marker #item).into()
}

/// 声明插件定时任务函数。
///
/// # 参数
///
/// - `cron = "..."`：必填，cron 表达式。
/// - `name = "..."`：可选，任务名；默认从函数名推导。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 函数不能有参数。
/// - 函数必须返回 `puniyu_plugin::Result`。
/// - `cron` 必须能被 `croner::Cron` 解析。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::task;
///
/// #[task(cron = "0 * * * * *", name = "health_check")]
/// async fn health_check() -> puniyu_plugin::Result {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn task(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match TaskArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::task(item, cfg).into()
}

/// 声明插件 Hook 函数。
///
/// # 参数
///
/// - `name = "..."`：可选，Hook 名；默认从函数名推导。
/// - `hook_type = "..."`：可选，也可写作 `type` 或 `r#type`；默认使用 `HookType::default()`。
/// - `priority = 100`：可选，优先级；默认 `500`。
///
/// `hook_type` 支持：`event`、`event.message`、`event.extension`、`event.all`、`status`、`status.start`、`status.stop`。
///
/// # 约束
///
/// - 被标注项必须是 `async fn`。
/// - 函数必须接收一个 `&HookType` 参数。
/// - 函数必须返回 `puniyu_plugin::Result`。
///
/// # 生成行为
///
/// - 生成 Hook 包装结构体并实现 `puniyu_plugin::__private::Hook`。
/// - 通过插件侧 `HookRegistry` 注册 Hook 构造器。
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_macros::plugin_hook;
/// use puniyu_plugin::hook::HookType;
///
/// #[plugin_hook(name = "on_message", hook_type = "event.message", priority = 100)]
/// async fn on_message(_event: &HookType) -> puniyu_plugin::Result {
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn plugin_hook(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = match parse_attr::<syn::ItemFn>(item) {
		Ok(item) => item,
		Err(err) => return err,
	};
	let cfg = match HookArgs::parse_tokens(args.into()) {
		Ok(cfg) => cfg,
		Err(err) => return err.to_compile_error().into(),
	};
	plugin::hook(item, cfg).into()
}
