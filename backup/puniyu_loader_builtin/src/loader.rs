use std::sync::Arc;

use async_trait::async_trait;
use puniyu_adapter_core::{Adapter, AdapterHandle};
use puniyu_loader::{
	Components, ComponentSource, DiscoveredAdapter, DiscoveredPlugin, DiscoveryMeta, LoadContext,
	Loader,
};
use puniyu_plugin_core::{Plugin, PluginHandle};

pub struct BuiltinLoader {
	adapters: Vec<AdapterHandle>,
	plugins: Vec<PluginHandle>,
}

impl BuiltinLoader {
	pub fn new() -> Self {
		Self { adapters: Vec::new(), plugins: Vec::new() }
	}

	pub fn with_adapter<A: Adapter + 'static>(mut self, adapter: A) -> Self {
		self.adapters.push(AdapterHandle::new(Arc::new(adapter)));
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

	async fn discover(&self, _ctx: &LoadContext) -> puniyu_error::Result<Components> {
		let adapters = self
			.adapters
			.iter()
			.map(|h| DiscoveredAdapter {
				handle: h.clone(),
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
				handle: p.clone(),
				meta: DiscoveryMeta {
					source: ComponentSource::Builtin,
					priority: 0,
				},
			})
			.collect();

		Ok(Components { adapters, plugins })
	}
}
