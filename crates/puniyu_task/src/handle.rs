use crate::Task;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct TaskHandle {
	inner: Arc<RwLock<Arc<dyn Task>>>,
}

impl TaskHandle {
	pub fn new(task: Arc<dyn Task>) -> Self {
		Self { inner: Arc::new(RwLock::new(task)) }
	}

	pub fn get(&self) -> Arc<dyn Task> {
		self.inner.read().expect("TaskHandle lock poisoned").clone()
	}

	pub fn set(&self, task: Arc<dyn Task>) -> Arc<dyn Task> {
		let mut guard = self.inner.write().expect("TaskHandle lock poisoned");
		std::mem::replace(&mut *guard, task)
	}
}

impl PartialEq for TaskHandle {
	fn eq(&self, other: &Self) -> bool {
		self.get() == other.get()
	}
}

impl std::fmt::Debug for TaskHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let task = self.get();
		f.debug_struct("TaskHandle")
			.field("name", &task.name())
			.finish()
	}
}
