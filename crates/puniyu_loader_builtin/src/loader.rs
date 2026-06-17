use std::sync::Arc;

use async_trait::async_trait;
use puniyu_adapter_core::{Adapter, AdapterHandle};
use puniyu_loader::{
	ComponentSet, ComponentSource, DiscoveredAdapter, DiscoveredPlugin, DiscoveryMeta, LoadContext,
	Loader,
};
use puniyu_plugin_core::{Plugin, PluginHandle};

struct BuiltinAdapter {
	adapter: Arc<dyn Adapter>,
	handle: AdapterHandle,
}

pub struct BuiltinLoader {
	adapters: Vec<BuiltinAdapter>,
	plugins: Vec<PluginHandle>,
}

impl BuiltinLoader {
	pub fn new() -> Self {
		Self { adapters: Vec::new(), plugins: Vec::new() }
	}

	pub fn with_adapter<A: Adapter + 'static>(mut self, adapter: A) -> Self {
		let arc: Arc<dyn Adapter> = Arc::new(adapter);
		let handle = AdapterHandle::new(arc.clone());
		self.adapters.push(BuiltinAdapter { adapter: arc, handle });
		self
	}

	pub fn with_plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
		self.plugins.push(PluginHandle::new(Arc::new(plugin)));
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
				adapter: a.adapter.clone(),
				handle: a.handle.clone(),
				meta: DiscoveryMeta {
					source: ComponentSource::Builtin,
					priority: 0,
				},
			})
			.collect();

		let plugins = self
			.plugins
			.iter()
			.map(|p| DiscoveredPlugin {
				instance: p.clone(),
				meta: DiscoveryMeta {
					source: ComponentSource::Builtin,
					priority: 0,
				},
			})
			.collect();

		Ok(ComponentSet { adapters, plugins })
	}
}
