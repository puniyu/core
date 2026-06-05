use std::sync::Arc;

use async_trait::async_trait;
use puniyu_adapter_core::Adapter;
use puniyu_loader::{
	ComponentSet, ComponentSource, DiscoveredAdapter, DiscoveredPlugin, DiscoveryMeta, LoadContext,
	Loader,
};
use puniyu_plugin_core::Plugin;

/// 内置加载器
///
/// 在编译期通过构建器模式注册 adapter 和 plugin。
/// `discover` 时将已注册的组件包装为带有 `DiscoveryMeta` 的发现结果。
///
/// # 示例
///
/// ```rust,ignore
/// let loader = BuiltinLoader::new()
///     .with_adapter(MyAdapter)
///     .with_plugin(MyPlugin);
/// ```
pub struct BuiltinLoader {
	adapters: Vec<Arc<dyn Adapter>>,
	plugins: Vec<Arc<dyn Plugin>>,
}

impl BuiltinLoader {
	/// 创建空的内置加载器
	pub fn new() -> Self {
		Self { adapters: Vec::new(), plugins: Vec::new() }
	}

	/// 添加适配器
	pub fn with_adapter<A: Adapter + 'static>(mut self, adapter: A) -> Self {
		self.adapters.push(Arc::new(adapter));
		self
	}

	/// 添加插件
	pub fn with_plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
		self.plugins.push(Arc::new(plugin));
		self
	}
}

impl Default for BuiltinLoader {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl Loader for BuiltinLoader {
	fn name(&self) -> &'static str {
		"builtin"
	}

	async fn discover(&self, _ctx: &LoadContext) -> puniyu_error::Result<ComponentSet> {
		let adapters = self
			.adapters
			.iter()
			.map(|a| DiscoveredAdapter {
				instance: Arc::clone(a),
				meta: DiscoveryMeta {
					loader_name: self.name(),
					source: ComponentSource::Builtin,
					priority: 0,
				},
			})
			.collect();

		let plugins = self
			.plugins
			.iter()
			.map(|p| DiscoveredPlugin {
				instance: Arc::clone(p),
				meta: DiscoveryMeta {
					loader_name: "builtin",
					source: ComponentSource::Builtin,
					priority: 0,
				},
			})
			.collect();

		Ok(ComponentSet { adapters, plugins })
	}
}
