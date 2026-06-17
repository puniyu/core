mod store;
use crate::HandlerId;
use crate::handle::HandlerHandle;
use puniyu_error::registry::Error;
use std::sync::LazyLock;
use store::HandlerStore;

static STORE: LazyLock<HandlerStore> = LazyLock::new(HandlerStore::new);

/// 处理器注册表。
pub struct HandlerRegistry;

impl<'h> HandlerRegistry {
	/// 注册处理器。
	pub fn register(handle: HandlerHandle) -> Result<u64, Error> {
		STORE.insert(handle)
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
		map.remove(&index);
		Ok(())
	}

	/// 按处理器名称卸载处理器。
	pub fn unregister_with_handler_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.retain(|_, handler| handler.get().name() != name);
		Ok(())
	}

	/// 获取处理器（按索引或名称）。
	pub fn get<H>(handler: H) -> Option<HandlerHandle>
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
	pub fn get_with_index(index: u64) -> Option<HandlerHandle> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 按名称获取处理器。
	pub fn get_with_handler_name(name: &str) -> Option<HandlerHandle> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|handler| handler.get().name() == name).cloned()
	}

	/// 获取所有处理器。
	pub fn all() -> Vec<HandlerHandle> {
		STORE.all()
	}
}
