//! # puniyu_macros
//!
//! `puniyu_macros` 提供 puniyu 生态中的过程宏，用于声明插件、命令、任务、配置和适配器入口。
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
//! ## 示例
//!
//! ```rust, ignore
//! use puniyu_plugin::prelude::*;
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

#[proc_macro_attribute]
pub fn arg(
	_args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	item
}

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
