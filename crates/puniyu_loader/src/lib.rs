//! # puniyu_loader
//!
//! Loader trait 定义及组件发现类型，作为 puniyu 系统中组件发现、解析和安装三层架构的第一层。
//!
//! ## 设计原则
//!
//! - Loader **只负责 discover**，不负责注册与生命周期
//! - Loader 是临时一次性的，不进入 registry，不长期持有组件
//! - core 统一负责 resolve 和 install

mod types;
#[doc(inline)]
pub use types::*;

use async_trait::async_trait;
use puniyu_error::Result;

/// 加载器 trait
///
/// 定义了组件的发现接口。每个加载器实现 `discover` 方法返回一组候选组件。
///
/// # 要求
///
/// - 必须是线程安全的（`Send + Sync`）
/// - 必须具有 `'static` 生命周期
///
/// # 示例
///
/// ```rust,ignore
/// use async_trait::async_trait;
/// use puniyu_loader::{Loader, LoadContext, ComponentSet};
///
/// struct MyLoader;
///
/// #[async_trait]
/// impl Loader for MyLoader {
///     fn name(&self) -> &'static str {
///         "my_loader"
///     }
///
///     async fn discover(&self, _ctx: &LoadContext) -> puniyu_error::Result<ComponentSet> {
///         Ok(ComponentSet {
///             adapters: vec![],
///             plugins: vec![],
///         })
///     }
/// }
/// ```
#[async_trait]
pub trait Loader: Send + Sync + 'static {
	/// 获取加载器名称
	fn name(&self) -> &'static str;

	/// 执行组件发现
	///
	/// 根据给定的上下文发现可用的适配器和插件。
	async fn discover(&self, ctx: &LoadContext) -> Result<ComponentSet>;
}
