mod config;
pub use config::config;
mod hook;
pub use hook::hook;

use crate::AdapterArgs;
use crate::common::{validate_async, validate_return_type};
use quote::quote;
use syn::ItemFn;

pub fn adapter(item: ItemFn, cfg: AdapterArgs) -> proc_macro2::TokenStream {
	if let Err(err) = validate_async(&item.sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&item.sig, "puniyu_adapter::Result") {
		return err.to_compile_error();
	}

	let fn_name = &item.sig.ident;
	let runtime = cfg.runtime;
	let server_impl = match cfg.server {
		Some(server) => quote! {
			fn server(&self) -> Option<::puniyu_adapter::__private::ServerFunction> {
				Some(::puniyu_adapter::__private::ServerFunction::new(#server))
			}
		},
		None => quote! {},
	};
	let init_impl = if item.block.stmts.is_empty() {
		quote! {}
	} else {
		quote! {
			#[inline]
			async fn init(&self) -> ::puniyu_adapter::Result {
				#fn_name().await
			}
		}
	};

	quote! {
		#item

		pub struct Adapter;

		#[::puniyu_adapter::__private::async_trait]
		impl ::puniyu_adapter::__private::Adapter for Adapter {
			fn runtime(&self) -> ::std::sync::Arc<dyn ::puniyu_adapter::runtime::AdapterRuntime> {
				#runtime()
			}

			fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::__private::Config>> {
				::puniyu_adapter::__private::inventory::iter::<crate::ConfigRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == env!("CARGO_PKG_NAME"))
					.map(|registry| (registry.builder)())
					.collect()
			}

			fn hook(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook>> {
				::puniyu_adapter::__private::inventory::iter::<crate::HookRegistry>
					.into_iter()
					.filter(|registry| registry.adapter_name == env!("CARGO_PKG_NAME"))
					.map(|registry| (registry.builder)())
					.collect()
			}

			#server_impl

			#init_impl
		}

		pub(crate) struct ConfigRegistry {
			adapter_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Config>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::ConfigRegistry);

		pub(crate) struct HookRegistry {
			adapter_name: &'static str,
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook>,
		}
		::puniyu_adapter::__private::inventory::collect!(crate::HookRegistry);
	}
}
