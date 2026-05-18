pub mod account;
pub mod bot;
pub mod config;
pub mod contact;
pub mod context;
pub mod element;
pub mod event;
pub mod message;
pub mod path;
pub mod runtime;
pub mod segment;
pub mod sender;
pub mod server;
pub mod version;

pub use inventory;
pub use puniyu_common::app::{app_name, app_version};

#[macro_export]
macro_rules! pkg_name {
	() => {{ env!("CARGO_PKG_NAME") }};
}
