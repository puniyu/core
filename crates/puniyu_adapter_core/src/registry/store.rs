use crate::AdapterHandle;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static ADAPTER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct AdapterStore(Arc<RwLock<HashMap<u64, AdapterHandle>>>);

impl AdapterStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, handle: AdapterHandle) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let adapter_name = handle.get().adapter_info().name.clone();
		if map.values().any(|v| v.get().adapter_info().name == adapter_name) {
			return Err(Error::Exists("Adapter".to_string()));
		}
		let index = ADAPTER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handle);
		Ok(index)
	}

	pub fn all(&self) -> Vec<AdapterHandle> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, AdapterHandle>>> {
		self.0.clone()
	}
}
