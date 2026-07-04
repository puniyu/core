use crate::Plugin;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct PluginHandle {
	inner: Arc<RwLock<Arc<dyn Plugin>>>,
}

impl PluginHandle {
	pub fn new(plugin: Arc<dyn Plugin>) -> Self {
		Self { inner: Arc::new(RwLock::new(plugin)) }
	}

	pub fn get(&self) -> Arc<dyn Plugin> {
		self.inner.read().expect("PluginHandle lock poisoned").clone()
	}

	pub fn set(&self, plugin: Arc<dyn Plugin>) -> Arc<dyn Plugin> {
		let mut guard = self.inner.write().expect("PluginHandle lock poisoned");
		std::mem::replace(&mut *guard, plugin)
	}

	pub fn strong_count(&self) -> usize {
		Arc::strong_count(&self.inner.read().expect("PluginHandle lock poisoned"))
	}
}

impl PartialEq for PluginHandle {
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl std::fmt::Debug for PluginHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let plugin = self.get();
		f.debug_struct("PluginHandle")
			.field("name", &plugin.name())
			.field("version", &plugin.version())
			.finish()
	}
}
