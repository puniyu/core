//! # puniyu_loader
//!
//! 加载器类型定义库，提供插件加载器的类型系统和注册管理。
//!
//! ## 概述
//!
//! `puniyu_loader` 提供了统一的加载器类型定义，用于管理聊天机器人中的插件加载器。
//! 加载器负责加载和管理一组相关的插件，提供插件的生命周期管理。
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保加载器的正确性
//! - 🔧 **注册管理** - 提供全局注册表管理加载器实例（需启用 `registry` 特性）
//! - 🔄 **统一接口** - 通过 `Loader` trait 提供统一的访问接口
//! - 📦 **插件管理** - 每个加载器可以管理多个插件
//! - 🎨 **线程安全** - 所有加载器都是线程安全的
//!
//! ## 使用方式
//!
//! ### 实现自定义加载器
//!
//! ```rust, ignore
//! use puniyu_loader::Loader;
//! use puniyu_plugin::Plugin;
//!
//! struct MyLoader;
//!
//! impl Loader for MyLoader {
//!     fn name(&self) -> &'static str {
//!         "my_loader"
//!     }
//!
//!     fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
//!         // 返回加载的插件列表
//!         vec![]
//!     }
//! }
//! ```
//!
//! ### 使用加载器注册表（需启用 `registry` 特性）
//!
//! ```rust,ignore
//! use puniyu_loader::{Loader, LoaderRegistry};
//! use std::sync::Arc;
//!
//! // 注册加载器
//! let loader = Arc::new(MyLoader);
//! let index = LoaderRegistry::register(loader)?;
//!
//! // 获取加载器
//! let loaders = LoaderRegistry::get(index);
//!
//! // 注销加载器
//! LoaderRegistry::unregister(index)?;
//! ```

#[cfg(feature = "registry")]
mod registry;

use async_trait::async_trait;
#[cfg(feature = "registry")]
pub use registry::LoaderRegistry;
use std::sync::Arc;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_plugin_core::Plugin;

/// 加载器 trait
///
/// 定义了加载器的基本接口，所有加载器都必须实现此 trait。
///
/// # 要求
///
/// - 必须是线程安全的（`Send + Sync`）
/// - 必须具有 `'static` 生命周期
///
/// # 示例
///
/// ```rust, ignore
/// use puniyu_loader::Loader;
/// use puniyu_plugin::Plugin;
///
/// struct MyLoader;
///
/// impl Loader for MyLoader {
///     fn name(&self) -> &'static str {
///         "my_loader"
///     }
///
///     fn plugins(&self) -> Vec<Arc<dyn Plugin>> {
///         vec![]
///     }
/// }
/// ```
#[async_trait]
pub trait Loader: Send + Sync + 'static {
	/// 获取加载器名称
	///
	/// 返回加载器的唯一标识名称。
	fn name(&self) -> &'static str;

	/// 获取加载器管理的插件列表
	///
	/// 返回此加载器加载的所有插件。
	async fn plugins(&self) -> Vec<Arc<dyn Plugin>>;
}

impl PartialEq for dyn Loader {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}
