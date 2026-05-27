mod store;

use crate::Adapter;
use crate::types::AdapterId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::AdapterStore;

static STORE: LazyLock<AdapterStore> = LazyLock::new(AdapterStore::new);

/// 适配器注册表。
pub struct AdapterRegistry;

impl<'a> AdapterRegistry {
	/// 注册适配器，并返回内部索引。
	pub fn register(adapter: Arc<dyn Adapter>) -> Result<u64, Error> {
		STORE.insert(adapter)
	}

	/// 按索引或名称卸载适配器。
	pub fn unregister<A>(adapter: A) -> Result<(), Error>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::unregister_with_index(index),
			AdapterId::Name(name) => Self::unregister_with_adapter_name(name.as_ref()),
		}
	}

	/// 通过索引卸载适配器。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	/// 通过名称卸载适配器。
	pub fn unregister_with_adapter_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.info().name == name) {
			return Err(Error::NotFound("Adapter".to_string()));
		}
		map.retain(|_, v| v.info().name != name);
		Ok(())
	}

	/// 按索引或名称查询适配器。
	pub fn get<A>(adapter: A) -> Vec<Arc<dyn Adapter>>
	where
		A: Into<AdapterId<'a>>,
	{
		let adapter = adapter.into();
		match adapter {
			AdapterId::Index(index) => Self::get_with_index(index).into_iter().collect(),
			AdapterId::Name(name) => Self::get_with_adapter_name(name.as_ref()),
		}
	}

	/// 通过索引查询适配器。
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 通过名称查询适配器。
	pub fn get_with_adapter_name(name: &str) -> Vec<Arc<dyn Adapter>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.info().name == name).cloned().collect()
	}

	/// 获取所有已注册适配器。
	pub fn all() -> Vec<Arc<dyn Adapter>> {
		STORE.all()
	}
}
