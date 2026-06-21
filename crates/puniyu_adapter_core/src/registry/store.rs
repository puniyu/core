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
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, adapter: Arc<dyn Adapter>) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		let adapter_name = adapter.adapter_info().name.clone();
		if map.values().any(|v| v.adapter_info().name == adapter_name) {
			return Err(Error::Exists("Adapter".to_string()));
		}
		let index = ADAPTER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, adapter);
		Ok(index)
	}

	pub fn all(&self) -> Vec<Arc<dyn Adapter>> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, Arc<dyn Adapter>>>> {
		self.0.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bytes::Bytes;
	use puniyu_account::AccountInfo;
	use puniyu_adapter_api::AdapterApi;
	use puniyu_adapter_types::{AdapterInfo, SendMsgType};
	use puniyu_contact::ContactType;
	use puniyu_message::Message;
	use std::time::Duration;

	struct MockAdapter {
		info: AdapterInfo,
		account: AccountInfo,
	}

	impl MockAdapter {
		fn new(name: &str) -> Self {
			Self {
				info: AdapterInfo::builder().name(name).build(),
				account: AccountInfo::builder().uin("0").name("test").avatar(Bytes::new()).build(),
			}
		}
	}

	#[async_trait::async_trait]
	impl AdapterApi for MockAdapter {
		async fn send_message(
			&self,
			_contact: &ContactType<'_>,
			_message: &Message,
		) -> puniyu_error::Result<SendMsgType> {
			Ok(SendMsgType { message_id: "0".into(), time: Duration::from_secs(0) })
		}

		fn adapter_info(&self) -> AdapterInfo {
			self.info.clone()
		}

		fn account_info(&self) -> AccountInfo {
			self.account.clone()
		}

		async fn call_api(
			&self,
			_action: &str,
			_params: serde_json::Value,
		) -> puniyu_error::Result<puniyu_common::Response<serde_json::Value>> {
			unimplemented!("mock")
		}
	}

	impl Adapter for MockAdapter {}

	#[test]
	fn store_new_creates_empty_store() {
		let store = AdapterStore::new();
		assert!(store.all().is_empty());
	}

	#[test]
	fn store_insert_returns_increasing_indices() {
		let store = AdapterStore::new();
		let a1 = Arc::new(MockAdapter::new("adapter_1"));
		let a2 = Arc::new(MockAdapter::new("adapter_2"));

		let idx1 = store.insert(a1).expect("first insert should succeed");
		let idx2 = store.insert(a2).expect("second insert should succeed");

		assert!(idx2 > idx1, "indices should be monotonically increasing");
	}

	#[test]
	fn store_insert_rejects_duplicate_name() {
		let store = AdapterStore::new();
		let a1 = Arc::new(MockAdapter::new("duplicate_name"));
		let a2 = Arc::new(MockAdapter::new("duplicate_name"));

		store.insert(a1).expect("first insert should succeed");
		let result = store.insert(a2);

		assert!(result.is_err());
		match result {
			Err(Error::Exists(msg)) => assert!(msg.contains("Adapter")),
			_ => panic!("Expected Error::Exists"),
		}
	}

	#[test]
	fn store_all_returns_all_inserted_adapters() {
		let store = AdapterStore::new();
		let a1 = Arc::new(MockAdapter::new("store_a"));
		let a2 = Arc::new(MockAdapter::new("store_b"));

		store.insert(a1).unwrap();
		store.insert(a2).unwrap();

		let all = store.all();
		assert_eq!(all.len(), 2);
		let names: Vec<String> =
			all.iter().map(|a| a.adapter_info().name.clone().to_string()).collect();
		assert!(names.iter().any(|n| n == "store_a"));
		assert!(names.iter().any(|n| n == "store_b"));
	}

	#[test]
	fn store_raw_provides_access_to_underlying_map() {
		let store = AdapterStore::new();
		let adapter = Arc::new(MockAdapter::new("raw_test"));
		store.insert(adapter).unwrap();

		let raw = store.raw();
		let map = raw.read().expect("Failed to acquire lock");
		assert!(!map.is_empty());
	}
}
