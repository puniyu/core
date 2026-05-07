mod command;
pub use command::command;
mod task;
pub use task::task;
mod config;
pub use config::config;
mod hook;
use crate::{
	PluginArg,
	common::{validate_async, validate_return_type},
};
pub use hook::hook;
use quote::quote;
use syn::ItemFn;

pub fn plugin(item: ItemFn, cfg: PluginArg) -> proc_macro2::TokenStream {
	if let Err(err) = validate_async(&item.sig) {
		return err.to_compile_error();
	}
	if !item.block.stmts.is_empty()
		&& let Err(err) = validate_return_type(&item.sig, "puniyu_plugin::Result")
	{
		return err.to_compile_error();
	}

	let plugin_desc = match cfg.desc {
		Some(desc) => quote!(Some(#desc)),
		None => quote!(None),
	};
	let plugin_prefix = match cfg.prefix {
		Some(prefix) => quote!(Some(#prefix)),
		None => quote!(None),
	};
	let fn_name = &item.sig.ident;
	let server_impl = match cfg.server {
		Some(server) => quote! {
			fn server(&self) -> Option<::puniyu_plugin::__private::ServerFunction> {
				Some(::puniyu_plugin::__private::ServerFunction::new(#server))
			}
		},
		None => quote! {},
	};
	let init_call = if item.block.stmts.is_empty() {
		quote! {}
	} else {
		quote! {
			#[inline]
			async fn init(&self) -> ::puniyu_plugin::Result {
				#fn_name().await
			}
		}
	};

	quote! {
		#item

		pub struct Plugin;

		#[::puniyu_plugin::__private::async_trait]
		impl ::puniyu_plugin::__private::Plugin for Plugin {
			fn name(&self) -> &str {
				env!("CARGO_PKG_NAME")
			}

			fn version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::Version {
					major: env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>().unwrap(),
					minor: env!("CARGO_PKG_VERSION_MINOR").parse::<u64>().unwrap(),
					patch: env!("CARGO_PKG_VERSION_PATCH").parse::<u64>().unwrap(),
				}
			}

			fn author(&self) -> ::std::vec::Vec<&str> {
				let authors = env!("CARGO_PKG_AUTHORS");
				if authors.is_empty() {
					::std::vec![]
				} else {
					authors.split(':').map(|author| author.trim()).collect::<::std::vec::Vec<&'static str>>()
				}
			}

			fn description(&self) -> Option<&str> {
				#plugin_desc
			}

			fn prefix(&self) -> Option<&str> {
				#plugin_prefix
			}

			fn tasks(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Task>> {
				::puniyu_plugin::__private::inventory::iter::<TaskRegistry>
					.into_iter()
					.filter(|task| task.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Command>> {
				::puniyu_plugin::__private::inventory::iter::<CommandRegistry>
					.into_iter()
					.filter(|command| command.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|command| (command.builder)())
					.collect()
			}

			fn hooks(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Hook>> {
				::puniyu_plugin::__private::inventory::iter::<HookRegistry>
					.into_iter()
					.filter(|hook| hook.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|hook| (hook.builder)())
					.collect()
			}

			fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Config>> {
				::puniyu_plugin::__private::inventory::iter::<ConfigRegistry>
					.into_iter()
					.filter(|config| config.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|config| (config.builder)())
					.collect()
			}

			#server_impl

			#init_call
		}

		pub(crate) struct TaskRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Task>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::TaskRegistry);

		pub(crate) struct CommandRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Command>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::CommandRegistry);

		pub(crate) struct ConfigRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Config>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::ConfigRegistry);

		pub(crate) struct HookRegistry {
			plugin_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Hook>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::HookRegistry);
	}
}
