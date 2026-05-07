use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, Ident, Path, Result, Signature, Type, spanned::Spanned};

pub(crate) fn validate_async(fn_sig: &Signature) -> Result<()> {
	if fn_sig.asyncness.is_none() {
		return Err(syn::Error::new(fn_sig.span(), "function must be async"));
	}
	Ok(())
}

pub(crate) fn validate_hook_args(fn_sig: &Signature) -> Result<()> {
	let arg_type = validate_single_ref_arg(fn_sig, "hook function parameter must not be `self`")?;
	if !path_matches(arg_type, &["HookType"])
		&& !path_matches(arg_type, &["puniyu_plugin", "hook", "HookType"])
		&& !path_matches(arg_type, &["puniyu_adapter", "hook", "HookType"])
	{
		return Err(syn::Error::new(
			arg_type.span(),
			"hook function parameter type must be `puniyu_plugin::hook::HookType` or `puniyu_adapter::hook::HookType`",
		));
	}
	Ok(())
}

pub(crate) fn validate_zero_args(fn_sig: &Signature) -> Result<()> {
	if !fn_sig.inputs.is_empty() {
		return Err(syn::Error::new(fn_sig.span(), "function must not have parameters"));
	}
	Ok(())
}

pub(crate) fn validate_single_ref_arg<'a>(
	fn_sig: &'a Signature,
	receiver_err: &str,
) -> Result<&'a Type> {
	if fn_sig.inputs.len() != 1 {
		return Err(syn::Error::new(
			fn_sig.span(),
			format!(
				"function `{}` must have exactly 1 parameter, found {}",
				fn_sig.ident,
				fn_sig.inputs.len()
			),
		));
	}

	let arg = fn_sig.inputs.first().expect("checked len");
	let pat_type = match arg {
		FnArg::Typed(pat_type) => pat_type,
		FnArg::Receiver(_) => return Err(syn::Error::new(arg.span(), receiver_err)),
	};

	match pat_type.ty.as_ref() {
		Type::Reference(type_ref) => Ok(type_ref.elem.as_ref()),
		_ => {
			Err(syn::Error::new(pat_type.ty.span(), "function parameter must be a reference type"))
		}
	}
}

pub(crate) fn validate_return_type(fn_sig: &Signature, expected: &str) -> Result<()> {
	let expected = expected.replace(' ', "");
	let err = |span| Err(syn::Error::new(span, format!("function must return `{expected}`")));

	let syn::ReturnType::Type(_, ty) = &fn_sig.output else {
		return err(fn_sig.span());
	};
	let Type::Path(type_path) = ty.as_ref() else {
		return err(ty.span());
	};
	let actual = normalize_path_string(&type_path.path);
	if actual == expected || actual == format!("::{expected}") { Ok(()) } else { err(ty.span()) }
}

pub(crate) fn function_struct_ident(fn_name: &Ident, suffix: &str) -> Ident {
	Ident::new(&format!("{}{}", fn_name.to_string().to_case(Case::Pascal), suffix), fn_name.span())
}

pub(crate) fn default_name_from_ident(ident: &Ident) -> String {
	ident.to_string().to_case(Case::Snake)
}

pub(crate) fn config_name(explicit_name: Option<&str>, ident: &Ident) -> String {
	explicit_name
		.map(|name| name.to_case(Case::Snake))
		.unwrap_or_else(|| default_name_from_ident(ident))
}

pub(crate) fn hook_type_tokens(
	root: &str,
	hook_type: Option<&str>,
	span: proc_macro2::Span,
) -> Result<TokenStream> {
	let root = match root {
		"plugin" => quote!(::puniyu_plugin::hook),
		"adapter" => quote!(::puniyu_adapter::hook),
		_ => return Err(syn::Error::new(span, "invalid hook root")),
	};

	let Some(hook_type) = hook_type else {
		return Ok(quote!(#root::HookType::default()));
	};

	let parts: Vec<&str> = hook_type.split('.').collect();
	match parts.as_slice() {
		["event"] => Ok(quote!(#root::HookType::Event(#root::HookEventType::default()))),
		["event", "message"] => Ok(quote!(#root::HookType::Event(#root::HookEventType::Message))),
		["event", "extension"] => {
			Ok(quote!(#root::HookType::Event(#root::HookEventType::Extension)))
		}
		["event", "all"] => Ok(quote!(#root::HookType::Event(#root::HookEventType::All))),
		["status"] => Ok(quote!(#root::HookType::Status(#root::StatusType::default()))),
		["status", "start"] => Ok(quote!(#root::HookType::Status(#root::StatusType::Start))),
		["status", "stop"] => Ok(quote!(#root::HookType::Status(#root::StatusType::Stop))),
		["event", subtype] => Err(syn::Error::new(
			span,
			format!(
				"Invalid event subtype '{subtype}'. Valid event subtypes are: 'message', 'extension', 'all'. Examples: 'event.message', 'event.all'"
			),
		)),
		["status", subtype] => Err(syn::Error::new(
			span,
			format!(
				"Invalid status subtype '{subtype}'. Valid status subtypes are: 'start', 'stop'. Examples: 'status.start', 'status.stop'"
			),
		)),
		[category, _] => Err(syn::Error::new(
			span,
			format!(
				"Invalid hook category '{category}'. Valid categories are: 'event', 'status'. Examples: 'event.message', 'status.start'"
			),
		)),
		[category] => Err(syn::Error::new(
			span,
			format!(
				"Invalid hook category '{category}'. Valid categories are: 'event', 'status'. Examples: 'event', 'event.message', 'status.start'"
			),
		)),
		_ => Err(syn::Error::new(
			span,
			format!(
				"Invalid hook type format '{hook_type}'. Expected format: 'category' or 'category.subtype'. Examples: 'event', 'event.message', 'status.start'"
			),
		)),
	}
}

pub(crate) fn path_matches(ty: &Type, expected_segments: &[&str]) -> bool {
	let Type::Path(type_path) = ty else {
		return false;
	};
	let actual: Vec<String> =
		type_path.path.segments.iter().map(|segment| segment.ident.to_string()).collect();
	if actual == expected_segments.iter().map(|segment| segment.to_string()).collect::<Vec<_>>() {
		return true;
	}
	if actual
		.last()
		.is_some_and(|segment| segment == expected_segments.last().copied().unwrap_or_default())
		&& expected_segments.len() == 1
	{
		return true;
	}
	false
}

pub(crate) fn normalize_path_string(path: &Path) -> String {
	path.to_token_stream().to_string().replace(' ', "")
}
