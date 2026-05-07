use crate::{
	ArgType, CommandArgs,
	common::{function_struct_ident, path_matches, validate_async},
};
use quote::{ToTokens, quote};
use syn::{Attribute, ItemFn, Signature, Type, spanned::Spanned};

pub fn command(mut item: ItemFn, cfg: CommandArgs) -> proc_macro2::TokenStream {
	let fn_sig = item.sig.clone();
	if let Err(err) = validate_async(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_command_args(&fn_sig) {
		return err.to_compile_error();
	}
	if let Err(err) = validate_command_return_type(&fn_sig) {
		return err.to_compile_error();
	}

	let args = match collect_arg_attrs(&mut item.attrs) {
		Ok(args) => args,
		Err(err) => return err.to_compile_error(),
	};

	let fn_name = &item.sig.ident;
	let struct_name = function_struct_ident(fn_name, "Command");
	let command_name = cfg.name;
	let command_priority = cfg.priority.unwrap_or(500u32);
	let command_desc = match cfg.desc {
		Some(desc) => quote!(Some(#desc)),
		None => quote!(None),
	};
	let command_permission = match cfg.permission.as_deref().unwrap_or("all") {
		"all" => quote!(::puniyu_plugin::command::Permission::All),
		"master" => quote!(::puniyu_plugin::command::Permission::Master),
		"owner" => quote!(::puniyu_plugin::command::Permission::Owner),
		"admin" => quote!(::puniyu_plugin::command::Permission::Admin),
		invalid => {
			return syn::Error::new_spanned(
				&item.sig.ident,
				format!("invalid command permission: {invalid}"),
			)
			.to_compile_error();
		}
	};
	let command_alias = cfg.alias.unwrap_or_default();
	let args_tokens = match build_arg_tokens(&args) {
		Ok(tokens) => tokens,
		Err(err) => return err.to_compile_error(),
	};

	quote! {
		#item

		struct #struct_name;

		#[::puniyu_plugin::__private::async_trait]
		impl ::puniyu_plugin::__private::Command for #struct_name {
			fn name(&self) -> &str {
				#command_name
			}

			fn description(&self) -> Option<&str> {
				#command_desc
			}

			fn priority(&self) -> u32 {
				#command_priority
			}

			fn args(&self) -> ::std::vec::Vec<::puniyu_plugin::command::Arg<'_>> {
				::std::vec![#(#args_tokens),*]
			}

			fn alias(&self) -> ::std::vec::Vec<&str> {
				::std::vec![#(#command_alias),*]
			}

			fn permission(&self) -> ::puniyu_plugin::command::Permission {
				#command_permission
			}

			#[inline]
			async fn execute(
				&self,
				ctx: &::puniyu_plugin::context::MessageContext,
			) -> ::puniyu_plugin::Result<::puniyu_plugin::command::CommandAction> {
				#fn_name(ctx).await
			}
		}

		::puniyu_plugin::__private::inventory::submit! {
			crate::CommandRegistry {
				plugin_name: env!("CARGO_PKG_NAME"),
				builder: || -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Command> {
					::std::sync::Arc::new(#struct_name {})
				}
			}
		}
	}
}

fn collect_arg_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<Vec<ArgType>> {
	let mut parsed = Vec::new();
	let mut retained = Vec::new();

	for attr in attrs.drain(..) {
		if attr.path().is_ident("arg") {
			let arg = attr.parse_args::<ArgType>()?;
			parsed.push(arg);
		} else {
			retained.push(attr);
		}
	}

	*attrs = retained;
	Ok(parsed)
}

fn build_arg_tokens(args: &[ArgType]) -> syn::Result<Vec<proc_macro2::TokenStream>> {
	args.iter()
		.map(|arg| {
			let name = &arg.name;
			let constructor = match arg.arg_type.as_deref().unwrap_or("string") {
				"string" => quote!(::puniyu_plugin::command::Arg::string(#name)),
				"integer" | "int" => quote!(::puniyu_plugin::command::Arg::int(#name)),
				"float" => quote!(::puniyu_plugin::command::Arg::float(#name)),
				"boolean" | "bool" => quote!(::puniyu_plugin::command::Arg::bool(#name)),
				invalid => {
					return Err(syn::Error::new(
						proc_macro2::Span::call_site(),
						format!("invalid arg type: {invalid}"),
					));
				}
			};
			let mode_method = match arg.mode.as_deref().unwrap_or("positional") {
				"positional" => quote!(.positional()),
				"named" | "optional" => quote!(.named()),
				invalid => {
					return Err(syn::Error::new(
						proc_macro2::Span::call_site(),
						format!("invalid arg mode: {invalid}"),
					));
				}
			};
			let required_method = if arg.required.unwrap_or(false) {
				quote!(.required())
			} else {
				quote!()
			};
			let desc_method = match &arg.desc {
				Some(desc) => quote!(.description(#desc)),
				None => quote!(),
			};
			Ok(quote!(#constructor #mode_method #required_method #desc_method))
		})
		.collect()
}

fn validate_command_args(fn_sig: &Signature) -> syn::Result<()> {
	let arg_type = crate::common::validate_single_ref_arg(fn_sig, "command function parameter must not be `self`")?;
	if !path_matches(arg_type, &["MessageContext"])
		&& !path_matches(arg_type, &["puniyu_context", "MessageContext"])
		&& !path_matches(arg_type, &["puniyu_plugin", "context", "MessageContext"])
	{
		return Err(syn::Error::new(
			arg_type.span(),
			"command function parameter type must be `MessageContext`, `puniyu_plugin::context::MessageContext` or `puniyu_context::MessageContext`",
		));
	}
	Ok(())
}

fn validate_command_return_type(fn_sig: &Signature) -> syn::Result<()> {
	let err = |span| {
		Err(syn::Error::new(
			span,
			"command function must return `puniyu_plugin::Result<CommandAction>` or `puniyu_plugin::Result<puniyu_plugin::command::CommandAction>`",
		))
	};
	let syn::ReturnType::Type(_, ty) = &fn_sig.output else {
		return err(fn_sig.span());
	};
	let Type::Path(tp) = ty.as_ref() else {
		return err(ty.span());
	};
	let actual = tp.path.to_token_stream().to_string().replace(' ', "");
	if matches!(
		actual.as_str(),
		"puniyu_plugin::Result<CommandAction>"
			| "::puniyu_plugin::Result<CommandAction>"
			| "puniyu_plugin::Result<puniyu_plugin::command::CommandAction>"
			| "::puniyu_plugin::Result<puniyu_plugin::command::CommandAction>"
			| "puniyu_plugin::Result<::puniyu_plugin::command::CommandAction>"
			| "::puniyu_plugin::Result<::puniyu_plugin::command::CommandAction>"
	) {
		Ok(())
	} else {
		err(ty.span())
	}
}
