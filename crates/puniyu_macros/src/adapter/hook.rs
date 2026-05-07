use crate::{
	HookArgs,
	common::{
		default_name_from_ident, function_struct_ident, hook_type_tokens, validate_async,
		validate_hook_args, validate_return_type,
	},
};
use quote::quote;
use syn::{ItemFn, LitStr, spanned::Spanned};

pub fn hook(item: ItemFn, cfg: HookArgs) -> proc_macro2::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_hook_args(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_return_type(&fn_sig, "puniyu_adapter::Result") {
		return err.to_compile_error();
	}

	let fn_name = &fn_sig.ident;
	let struct_name = function_struct_ident(fn_name, "Hook");
	let hook_name = cfg.name.unwrap_or_else(|| default_name_from_ident(fn_name));
	let hook_type_value = cfg.hook_type.as_ref().map(LitStr::value);
	let hook_type_span = cfg.hook_type.as_ref().map_or_else(|| fn_sig.span(), LitStr::span);
	let hook_type = match hook_type_tokens("adapter", hook_type_value.as_deref(), hook_type_span) {
		Ok(tokens) => tokens,
		Err(err) => return err.to_compile_error(),
	};
	let hook_priority = cfg.priority.unwrap_or(500);

	quote! {
		#item

		struct #struct_name;

		#[::puniyu_adapter::__private::async_trait]
		impl ::puniyu_adapter::__private::Hook for #struct_name {
			fn name(&self) -> &'static str {
				#hook_name
			}

			fn r#type(&self) -> &::puniyu_adapter::hook::HookType {
				static HOOK_TYPE: ::std::sync::LazyLock<::puniyu_adapter::hook::HookType> =
					::std::sync::LazyLock::new(|| #hook_type);
				&HOOK_TYPE
			}

			fn priority(&self) -> u32 {
				#hook_priority
			}

			#[inline]
			async fn execute(
				&self,
				_event: Option<&::puniyu_adapter::context::EventContext>,
			) -> ::puniyu_adapter::Result {
				#fn_name(self.r#type()).await
			}
		}

		::puniyu_adapter::__private::inventory::submit! {
			crate::HookRegistry {
				adapter_name: env!("CARGO_PKG_NAME"),
				builder: || -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Hook> {
					::std::sync::Arc::new(#struct_name {})
				}
			}
		}
	}
}
