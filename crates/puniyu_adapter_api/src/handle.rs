use crate::AdapterApi;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AdapterHandle {
	inner: Arc<RwLock<Arc<dyn AdapterApi>>>,
}

impl AdapterHandle {
	pub fn new(adapter: Arc<dyn AdapterApi>) -> Self {
		Self { inner: Arc::new(RwLock::new(adapter)) }
	}

	pub fn get(&self) -> Arc<dyn AdapterApi> {
		self.inner.read().expect("AdapterHandle lock poisoned").clone()
	}

	pub fn set(&self, adapter: Arc<dyn AdapterApi>) -> Arc<dyn AdapterApi> {
		let mut guard = self.inner.write().expect("AdapterHandle lock poisoned");
		std::mem::replace(&mut *guard, adapter)
	}
}

impl PartialEq for AdapterHandle {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.inner, &other.inner)
	}
}

impl std::fmt::Debug for AdapterHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let adapter = self.get();
		f.debug_struct("AdapterHandle")
			.field("name", &adapter.adapter_info().name)
			.finish()
	}
}
