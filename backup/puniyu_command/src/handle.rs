use crate::Command;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct CommandHandle {
	inner: Arc<RwLock<Arc<dyn Command>>>,
}

impl CommandHandle {
	pub fn new(command: Arc<dyn Command>) -> Self {
		Self { inner: Arc::new(RwLock::new(command)) }
	}

	pub fn get(&self) -> Arc<dyn Command> {
		self.inner.read().expect("CommandHandle lock poisoned").clone()
	}

	pub fn set(&self, command: Arc<dyn Command>) -> Arc<dyn Command> {
		let mut guard = self.inner.write().expect("CommandHandle lock poisoned");
		std::mem::replace(&mut *guard, command)
	}
}

impl PartialEq for CommandHandle {
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl std::fmt::Debug for CommandHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let cmd = self.get();
		f.debug_struct("CommandHandle")
			.field("name", &cmd.name())
			.finish()
	}
}
