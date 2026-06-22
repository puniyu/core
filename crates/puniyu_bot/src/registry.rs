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

	/// 按索引或 UIN 从注册表移除机器人。
	pub fn unregister<'b>(bot_id: impl Into<BotId<'b>>) -> Result<(), Error> {
		match bot_id.into() {
			BotId::Index(id) => Self::unregister_with_index(id),
			BotId::SelfId(id) => Self::unregister_with_bot_id(id.as_ref()),
		}
	}

	/// 按注册表索引移除机器人。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		if STORE.remove(index).is_none() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		Ok(())
	}

	/// 按机器人 UIN 移除所有匹配的机器人。
	pub fn unregister_with_bot_id(bot_id: &str) -> Result<(), Error> {
		let indices = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter()
				.filter_map(|(k, v)| if v.account_info().uin == bot_id { Some(*k) } else { None })
				.collect::<Vec<u64>>()
		};
		if indices.is_empty() {
			return Err(Error::NotFound("Bot".to_string()));
		}
		for idx in indices {
			STORE.remove(idx);
		}
		Ok(())
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
		map.get(&index).cloned()
	}

	/// 按机器人 UIN 查询第一个匹配的机器人。
	pub fn get_with_bot_id(self_id: &str) -> Option<Arc<Bot>> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().find(|bot| bot.account_info().uin == self_id).cloned()
	}

	/// 返回所有已注册的机器人。
	pub fn all() -> Vec<Arc<Bot>> {
		STORE.all()
	}
}
