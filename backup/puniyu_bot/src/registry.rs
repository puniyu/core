mod store;
use crate::Bot;
use crate::types::BotId;
use puniyu_error::registry::Error;
use std::sync::{Arc, LazyLock};
use store::BotStore;

static STORE: LazyLock<BotStore> = LazyLock::new(BotStore::new);

/// 全局机器人注册表。
pub struct BotRegistry;

impl BotRegistry {
	/// 将机器人注册到全局注册表，返回分配的索引。
	pub fn register(bot: Arc<Bot>) -> Result<u64, Error> {
		STORE.insert(bot)
	}

	/// 按索引或 UIN 从注册表移除机器人，返回旧实例供清理。
	pub fn unregister<'b>(bot_id: impl Into<BotId<'b>>) -> Result<Vec<Arc<Bot>>, Error> {
		match bot_id.into() {
			BotId::Index(id) => Self::unregister_with_index(id).map(|b| vec![b]),
			BotId::SelfId(id) => Self::unregister_with_bot_id(id.as_ref()),
		}
	}

	/// 按注册表索引移除机器人，返回旧实例供清理。
	pub fn unregister_with_index(index: u64) -> Result<Arc<Bot>, Error> {
		STORE.remove(index).ok_or_else(|| Error::NotFound("Bot".to_string()))
	}

	/// 按机器人 UIN 移除所有匹配的机器人，返回旧实例供清理。
	pub fn unregister_with_bot_id(bot_id: &str) -> Result<Vec<Arc<Bot>>, Error> {
		let removed = STORE.remove_by_uin(bot_id);
		if removed.is_empty() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		Ok(removed)
	}

	/// 按索引或 UIN 查询机器人。
	pub fn get<'b>(bot_id: impl Into<BotId<'b>>) -> Option<Arc<Bot>> {
		match bot_id.into() {
			BotId::Index(index) => Self::get_with_index(index),
			BotId::SelfId(self_id) => Self::get_with_bot_id(self_id.as_ref()),
		}
	}

	/// 按注册表索引查询机器人。
	pub fn get_with_index(index: u64) -> Option<Arc<Bot>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).map(|h| h.get())
	}

	/// 按机器人 UIN 查询第一个匹配的机器人。
	pub fn get_with_bot_id(self_id: &str) -> Option<Arc<Bot>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|h| h.get().account_info().uin == self_id).map(|h| h.get())
	}

	/// 返回所有已注册的机器人。
	pub fn all() -> Vec<Arc<Bot>> {
		STORE.all()
	}
}
