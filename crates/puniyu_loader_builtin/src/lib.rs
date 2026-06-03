//! # puniyu_loader_builtin
//!
//! Puniyu 内置加载器，用于在编译期通过构建器模式注册 adapter 和 plugin。
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! use puniyu_loader_builtin::BuiltinLoader;
//!
//! let loader = BuiltinLoader::new()
//!     .with_adapter(MyAdapter)
//!     .with_plugin(MyPlugin);
//! ```

mod loader;
pub use loader::BuiltinLoader;