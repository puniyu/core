use crate::Bot;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct BotHandle {
	inner: Arc<RwLock<Arc<Bot>>>,
}

impl BotHandle {
	pub fn new(bot: Arc<Bot>) -> Self {
		Self { inner: Arc::new(RwLock::new(bot)) }
	}

	pub fn get(&self) -> Arc<Bot> {
		self.inner.read().expect("BotHandle lock poisoned").clone()
	}

	pub fn set(&self, bot: Arc<Bot>) -> Arc<Bot> {
		let mut guard = self.inner.write().expect("BotHandle lock poisoned");
		std::mem::replace(&mut *guard, bot)
	}

	pub fn strong_count(&self) -> usize {
		Arc::strong_count(&self.inner.read().expect("BotHandle lock poisoned"))
	}
}

impl PartialEq for BotHandle {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.inner, &other.inner)
	}
}

impl std::fmt::Debug for BotHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let bot = self.get();
		f.debug_struct("BotHandle")
			.field("uin", &bot.self_id())
			.field("adapter", &bot.adapter_info().name)
			.finish()
	}
}
