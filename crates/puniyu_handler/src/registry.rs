mod store;
use crate::Handler;
use crate::HandlerId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::HandlerStore;

static STORE: LazyLock<HandlerStore> = LazyLock::new(HandlerStore::new);

/// 处理器注册表。
pub struct HandlerRegistry;

impl<'h> HandlerRegistry {
	/// 注册处理器。
	pub fn register(handler: Arc<dyn Handler>) -> Result<u64, Error> {
		STORE.insert(handler)
	}

	/// 卸载处理器（按索引或名称）。
	pub fn unregister<H>(handler: H) -> Result<(), Error>
	where
		H: Into<HandlerId<'h>>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::unregister_with_index(index),
			HandlerId::Name(name) => Self::unregister_with_handler_name(name.as_ref()),
		}
	}
	/// 按索引卸载处理器。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		match map.remove(&index) {
			Some(_) => Ok(()),
			None => Err(Error::NotFound(index.to_string())),
		}
	}

	/// 按处理器名称卸载处理器。
	pub fn unregister_with_handler_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let key = map.iter().find(|(_, h)| h.name() == name).map(|(k, _)| *k);
		match key.and_then(|k| map.remove(&k)) {
			Some(_) => Ok(()),
			None => Err(Error::NotFound(name.to_string())),
		}
	}

	/// 获取处理器（按索引或名称）。
	pub fn get<H>(handler: H) -> Option<Arc<dyn Handler>>
	where
		H: Into<HandlerId<'h>>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::get_with_index(index),
			HandlerId::Name(name) => Self::get_with_handler_name(name.as_ref()),
		}
	}
	/// 按索引获取处理器。
	pub fn get_with_index(index: u64) -> Option<Arc<dyn Handler>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 按名称获取处理器。
	pub fn get_with_handler_name(name: &str) -> Option<Arc<dyn Handler>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|handler| handler.name() == name).cloned()
	}

	/// 获取所有处理器。
	pub fn all() -> Vec<Arc<dyn Handler>> {
		STORE.all()
	}
}
