use crate::{
	TaskArgs,
	common::{
		default_name_from_ident, function_struct_ident, validate_async, validate_return_type,
		validate_zero_args,
	},
};
use croner::Cron;
use quote::quote;
use std::str::FromStr;
use syn::ItemFn;

pub fn task(item: ItemFn, cfg: TaskArgs) -> proc_macro2::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_zero_args(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&fn_sig, "puniyu_plugin::Result") {
		return err.to_compile_error();
	}

	let fn_name = &item.sig.ident;
	let cron_expr = cfg.cron;
	if Cron::from_str(&cron_expr.value()).is_err() {
		return syn::Error::new_spanned(&cron_expr, "Invalid cron expression format")
			.to_compile_error();
	}

	let task_name = cfg.name.unwrap_or_else(|| default_name_from_ident(fn_name));
	let struct_name = function_struct_ident(fn_name, "Task");

	quote! {
		#item

		struct #struct_name;

		#[::puniyu_plugin::__private::async_trait]
		impl ::puniyu_plugin::__private::Task for #struct_name {
			fn name(&self) -> &'static str {
				#task_name
			}

			fn cron(&self) -> &'static str {
				#cron_expr
			}

			#[inline]
			async fn execute(&self) -> ::puniyu_plugin::Result {
				#fn_name().await
			}
		}

		::puniyu_plugin::__private::inventory::submit! {
			crate::TaskRegistry {
				plugin_name: env!("CARGO_PKG_NAME"),
				builder: || -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Task> {
					::std::sync::Arc::new(#struct_name {})
				},
			}
		}
	}
}
