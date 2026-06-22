use crate::Bot;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub(crate) struct BotHandle {
	inner: Arc<RwLock<Arc<Bot>>>,
}

impl BotHandle {
	fn new(bot: Arc<Bot>) -> Self {
		Self { inner: Arc::new(RwLock::new(bot)) }
	}

	pub(crate) fn get(&self) -> Arc<Bot> {
		self.inner.read().expect("BotHandle lock poisoned").clone()
	}
}

#[derive(Clone, Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, BotHandle>>>);

impl BotStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, bot: Arc<Bot>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let bot_uin = &bot.account_info().uin;
		if map.values().any(|v| {
			let existing = v.get();
			Arc::ptr_eq(&existing, &bot) || existing.account_info().uin == *bot_uin
		}) {
			return Err(Error::Exists("Bot".to_string()));
		}
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, BotHandle::new(bot));
		Ok(index)
	}

	pub fn remove(&self, bot_index: u64) -> Option<Arc<Bot>> {
		self.0.write().expect("Failed to acquire lock").remove(&bot_index).map(|h| h.get())
	}

	pub fn remove_by_uin(&self, bot_id: &str) -> Vec<Arc<Bot>> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let keys: Vec<u64> = map
			.iter()
			.filter_map(|(k, v)| if v.get().account_info().uin == bot_id { Some(*k) } else { None })
			.collect();
		keys.into_iter().filter_map(|k| map.remove(&k).map(|h| h.get())).collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, BotHandle>>> {
		self.0.clone()
	}

	pub fn all(&self) -> Vec<Arc<Bot>> {
		self.0.read().expect("Failed to acquire lock").values().map(|h| h.get()).collect()
	}
}
