use puniyu_error::registry::Error;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use toml::Value;

static CONFIG_ID: AtomicU64 = AtomicU64::new(0);

/// 配置存储条目
#[derive(Debug, Clone)]
pub struct ConfigEntry {
	pub name: String,
	pub path: PathBuf,
	pub value: Value,
}

impl PartialEq for ConfigEntry {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.path == other.path
	}
}

/// 配置存储
///
/// 内部使用的配置存储结构，提供线程安全的配置管理。
#[derive(Default)]
pub(crate) struct ConfigStore(Arc<RwLock<HashMap<u64, ConfigEntry>>>);

impl ConfigStore {
	/// 创建新的配置存储
	pub fn new() -> Self {
		Self::default()
	}

	/// 插入配置
	pub fn insert(&self, name: String, path: PathBuf, value: Value) -> Result<u64, Error> {
		let entry = ConfigEntry { name, path, value };
		let mut map = self.0.write().expect("Failed to acquire lock");
		if map.values().any(|v| v == &entry) {
			return Err(Error::Exists("Config".to_string()));
		}
		let index = CONFIG_ID.fetch_add(1, Ordering::Relaxed);
		map.insert(index, entry);
		Ok(index)
	}

	/// 获取所有配置
	pub fn all(&self) -> Vec<ConfigEntry> {
		let map = self.0.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub(crate) fn raw(&self) -> Arc<RwLock<HashMap<u64, ConfigEntry>>> {
		self.0.clone()
	}
}
