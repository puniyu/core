use crate::Handler;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct HandlerHandle {
	inner: Arc<RwLock<Arc<dyn Handler>>>,
}

impl HandlerHandle {
	pub fn new(handler: Arc<dyn Handler>) -> Self {
		Self { inner: Arc::new(RwLock::new(handler)) }
	}

	pub fn get(&self) -> Arc<dyn Handler> {
		self.inner.read().expect("HandlerHandle lock poisoned").clone()
	}

	pub fn set(&self, handler: Arc<dyn Handler>) -> Arc<dyn Handler> {
		let mut guard = self.inner.write().expect("HandlerHandle lock poisoned");
		std::mem::replace(&mut *guard, handler)
	}

	pub fn strong_count(&self) -> usize {
		Arc::strong_count(&self.inner.read().expect("HandlerHandle lock poisoned"))
	}
}

impl PartialEq for HandlerHandle {
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl std::fmt::Debug for HandlerHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let handler = self.get();
		f.debug_struct("HandlerHandle")
			.field("name", &handler.name())
			.finish()
	}
}
