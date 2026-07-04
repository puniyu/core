use crate::types::{ServerId, ServerInfo};
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};

static SERVER_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Default)]
pub(crate) struct ServerStore(Arc<RwLock<HashMap<u64, ServerInfo>>>);

impl ServerStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, server: ServerInfo) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &server) {
			return Err(Error::Exists("Server".to_string()));
		}
		let id = SERVER_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(id, server);
		Ok(id)
	}

	pub fn find(&self, f: impl Fn(&ServerInfo) -> bool) -> Option<ServerInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().find(|v| f(v)).cloned()
	}

	pub fn all(&self) -> Vec<ServerInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn remove(&self, id: impl Into<ServerId>) -> Result<(), Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		match id.into() {
			ServerId::Index(index) => {
				map.remove(&index).map(|_| ()).ok_or_else(|| Error::NotFound("Server".to_string()))
			}
			ServerId::Source(source) => {
				let count = map.len();
				map.retain(|_, v| v.source != source);
				if map.len() == count { Err(Error::NotFound("Server".to_string())) } else { Ok(()) }
			}
		}
	}
}
