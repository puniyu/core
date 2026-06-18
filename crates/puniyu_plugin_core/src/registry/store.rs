use crate::handle::PluginHandle;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
static PLUGIN_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct PluginStore(Arc<RwLock<HashMap<u64, PluginHandle>>>);

impl PluginStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, handle: PluginHandle) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| *v == handle) {
			return Err(Error::Exists("Plugin".to_string()));
		}
		let index = PLUGIN_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handle);
		Ok(index)
	}

	pub fn all(&self) -> Vec<PluginHandle> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, PluginHandle>>> {
		self.0.clone()
	}
}
