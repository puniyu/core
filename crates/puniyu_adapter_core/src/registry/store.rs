use crate::Adapter;
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
pub(crate) struct AdapterStore(Arc<RwLock<HashMap<u64, Arc<dyn Adapter>>>>);

impl AdapterStore {
	/// 创建适配器存储。
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入适配器并返回内部索引。
	pub fn insert(&self, adapter: Arc<dyn Adapter>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let adapter_name = adapter.info().name.clone();
		if map.values().any(|v| v.info().name == adapter_name) {
			return Err(Error::Exists("Adapter".to_string()));
		}
		let index = ADAPTER_INDEX.fetch_add(1, Ordering::SeqCst);
		map.insert(index, adapter);
		Ok(index)
	}

	/// 获取所有适配器。
	pub fn all(&self) -> Vec<Arc<dyn Adapter>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	/// 获取原始存储引用。
	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Adapter>>>> {
		self.0.clone()
	}
}
