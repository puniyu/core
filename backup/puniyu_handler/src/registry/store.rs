use crate::Handler;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

static HANDLER_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct HandlerStore(pub(crate) Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>>);

impl HandlerStore {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn insert(&self, handler: Arc<dyn Handler>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v.name() == handler.name()) {
			return Err(Error::Exists("Handler".to_string()));
		}
		let index = HANDLER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handler);
		Ok(index)
	}

	pub fn all(&self) -> Vec<Arc<dyn Handler>> {
		let handlers = self.0.read().expect("Failed to acquire lock");
		handlers.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Handler>>>> {
		self.0.clone()
	}
}
