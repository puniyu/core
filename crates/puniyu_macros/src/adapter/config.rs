use crate::{ConfigArgs, common::config_name};
use quote::quote;
use syn::ItemStruct;

pub fn config(item: ItemStruct, cfg: ConfigArgs) -> proc_macro2::TokenStream {
	let struct_name = &item.ident;
	let config_name = config_name(cfg.name.as_deref(), struct_name);
	let config_file_name = format!("{config_name}.toml");
	let serialize_error = format!("Failed to serialize {config_name} to TOML string");

	quote! {
		#item

		impl ::puniyu_adapter::__private::Config for #struct_name {
			fn config(&self) -> ::puniyu_adapter::__private::ConfigInfo {
				::puniyu_adapter::__private::ConfigInfo {
					name: ::std::string::String::from(#config_name),
					path: ::puniyu_adapter::path::adapter::config_dir()
						.join(env!("CARGO_PKG_NAME"))
						.join(#config_file_name),
					value: ::puniyu_adapter::__private::toml::from_str(
						&::puniyu_adapter::__private::toml::to_string(self).expect(#serialize_error),
					)
					.expect("Failed to parse TOML string to Value"),
				}
			}
		}

		::puniyu_adapter::__private::inventory::submit! {
			crate::ConfigRegistry {
				adapter_name: env!("CARGO_PKG_NAME"),
				builder: || -> ::std::sync::Arc<dyn ::puniyu_adapter::__private::Config> {
					::std::sync::Arc::new(#struct_name::default())
				}
			}
		}
	}
}
