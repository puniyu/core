pub mod account;
pub mod bot;
pub mod command;
pub mod config;
pub mod contact;
pub mod context;
pub mod element;
pub mod event;
pub mod message;
pub mod path;
pub mod result;
pub mod runtime;
pub mod segment;
pub mod semver;
pub mod sender;
pub mod server;
pub mod task;

pub use inventory;
pub use tokio;
pub use toml;
pub use async_trait;
pub use puniyu_common::app::{app_name, app_version};

#[macro_export]
macro_rules! pkg_name {
	() => {{ env!("CARGO_PKG_NAME") }};
}


#[macro_export]
macro_rules! pkg_version {
	() => {{
		const fn parse(s: &str) -> u64 {
			const fn inner(bytes: &[u8], i: usize, r: u64) -> u64 {
				if i >= bytes.len() {
					r
				} else {
					inner(bytes, i + 1, r * 10 + (bytes[i] - b'0') as u64)
				}
			}
			inner(s.as_bytes(), 0, 0)
		}
		$crate::semver::Version::new(
			parse(env!("CARGO_PKG_VERSION_MAJOR")),
			parse(env!("CARGO_PKG_VERSION_MINOR")),
			parse(env!("CARGO_PKG_VERSION_PATCH")),
		)
	}};
}
