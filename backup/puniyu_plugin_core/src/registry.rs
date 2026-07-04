use std::sync::LazyLock;

mod store;
use crate::PluginId;
use crate::handle::PluginHandle;
use puniyu_error::registry::Error;
use store::PluginStore;

static STORE: LazyLock<PluginStore> = LazyLock::new(PluginStore::new);

#[derive(Debug, Default)]
pub struct PluginRegistry;
impl<'p> PluginRegistry {
	/// 注册一个插件
	pub fn register(handle: PluginHandle) -> Result<u64, Error> {
		STORE.insert(handle)
	}

	/// 卸载一个插件
	pub fn unregister<P>(plugin: P) -> Result<(), Error>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::unregister_with_index(index),
			PluginId::Name(name) => Self::unregister_with_plugin_name(name.as_ref()),
		}
	}

	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	pub fn unregister_with_plugin_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.get().name() == name) {
			return Err(Error::NotFound("Plugin".to_string()));
		}
		map.retain(|_, v| v.get().name() != name);
		Ok(())
	}

	pub fn get<P>(plugin: P) -> Option<PluginHandle>
	where
		P: Into<PluginId<'p>>,
	{
		let plugin_id = plugin.into();
		match plugin_id {
			PluginId::Index(index) => Self::get_with_index(index),
			PluginId::Name(name) => Self::get_with_plugin_name(name.as_ref()),
		}
	}

	pub fn get_with_index(index: u64) -> Option<PluginHandle> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	pub fn get_with_plugin_name(name: &str) -> Option<PluginHandle> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|v| v.get().name() == name).cloned()
	}

	pub fn all() -> Vec<PluginHandle> {
		STORE.all()
	}
}
