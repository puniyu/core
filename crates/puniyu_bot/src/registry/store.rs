use crate::handle::BotHandle;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static BOT_INDEX: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct BotStore(Arc<RwLock<HashMap<u64, BotHandle>>>);

impl BotStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, handle: BotHandle) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let bot_uin = &handle.get().account_info().uin;
		if map.values().any(|v| *v == handle || v.get().account_info().uin == *bot_uin) {
			return Err(Error::Exists("Bot".to_string()));
		}
		let index = BOT_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handle);
		Ok(index)
	}

	pub fn remove(&self, bot_index: u64) -> Option<BotHandle> {
		self.0.write().expect("Failed to acquire lock").remove(&bot_index)
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, BotHandle>>> {
		self.0.clone()
	}

	pub fn all(&self) -> Vec<BotHandle> {
		self.0.read().expect("Failed to acquire lock").values().cloned().collect()
	}
}
