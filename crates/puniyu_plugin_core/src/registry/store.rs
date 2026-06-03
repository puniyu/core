use crate::Plugin;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct PluginStore(Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>>);

impl PluginStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, plugin: Arc<dyn Plugin>) -> Result<u64, Error> {
		let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &plugin) {
			return Err(Error::Exists("Plugin".to_string()));
		}
		map.insert(index, plugin);
		Ok(index)
	}

	pub fn all(&self) -> Vec<Arc<dyn Plugin>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Plugin>>>> {
		self.0.clone()
	}
}
