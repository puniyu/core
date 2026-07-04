use crate::types::CommandInfo;
use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

static COMMAND_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub(crate) struct CommandStore(Arc<RwLock<HashMap<u64, CommandInfo>>>);

impl CommandStore {
	/// 创建命令存储。
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入命令并返回内部索引。
	pub fn insert(&self, command: CommandInfo) -> Result<u64, Error> {
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &command) {
			return Err(Error::Exists("Command".to_string()));
		}
		let index = COMMAND_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, command);
		Ok(index)
	}

	/// 获取所有命令。
	pub fn all(&self) -> Vec<CommandInfo> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	/// 获取原始存储引用。
	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, CommandInfo>>> {
		self.0.clone()
	}
}
