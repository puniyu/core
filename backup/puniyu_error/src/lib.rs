//! # puniyu_error
//!
//! 错误类型库，提供配置与注册表场景的统一错误定义。
//!
//! ## 特性
//!
//! - 提供 `config::Error` 与 `registry::Error`
//! - 支持 `config` / `registry` / `full` feature
//! - 提供通用 `Result<T>` 类型别名

#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "registry")]
pub mod registry;

/// 通用 Result 类型别名。
pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
