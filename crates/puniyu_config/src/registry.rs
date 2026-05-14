mod store;

use crate::types::ConfigId;
use puniyu_error::registry::Error;
use std::{fs, path::Path, sync::LazyLock};
use store::{ConfigEntry, ConfigStore};
use toml::Value;
use crate::logger::config_error;

static STORE: LazyLock<ConfigStore> = LazyLock::new(ConfigStore::new);

/// 配置注册表
pub struct ConfigRegistry;

impl ConfigRegistry {
	/// 注册配置到注册表。
	///
	/// 注册时自动读取用户配置文件并合并到默认配置中
	pub fn register<C: crate::Config>(config: C) -> Result<u64, Error> {
		let dir = config.path();
		let name = config.name();
		let file_path = dir.join(format!("{}.toml", name));

		let merged = Self::merge_with_file(&dir, name, &config.to_value());

		if let Some(parent) = file_path.parent()
			&& let Err(e) = fs::create_dir_all(parent) {
				config_error!("[Config] Failed to create config directory: {}", e);
			}
		if let Err(e) = fs::write(&file_path, toml::to_string_pretty(&merged).expect("Failed to serialize config")) {
			config_error!("[Config] Failed to write config file: {}", e);
		}

		STORE.insert(name.to_string(), file_path, merged)
	}

	pub fn register_entry(
		name: &str,
		path: std::path::PathBuf,
		value: Value,
	) -> Result<u64, Error> {
		STORE.insert(name.to_string(), path, value)
	}

	fn merge_with_file(dir: &Path, name: &str, default: &Value) -> Value {
		use puniyu_common::merge_toml_values;
		use puniyu_common::read_config;

		let mut merged = default.clone();
		if let Ok(file_value) = read_config::<Value>(dir, name) {
			merge_toml_values(&mut merged, file_value);
		}
		merged
	}

	pub fn get<C>(id: C) -> Option<Value>
	where
		C: Into<ConfigId>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::get_with_index(id),
			ConfigId::Path(path) => Self::get_with_path(path),
		}
	}

	pub fn get_with_index(id: u64) -> Option<Value> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.get(&id).cloned()?.value)
	}

	pub fn get_with_path<P>(path: P) -> Option<Value>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		Some(map.values().find(|v| v.path == path.as_ref())?.value.clone())
	}

	pub fn update<C>(id: C, value: Value) -> Result<(), Error>
	where
		C: Into<ConfigId>,
	{
		let id = id.into();
		match id {
			ConfigId::Index(id) => Self::update_with_index(id, value),
			ConfigId::Path(path) => Self::update_with_path(path, value),
		}
	}

	pub fn update_with_index(id: u64, value: Value) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let config = map.get_mut(&id).ok_or(Error::NotFound("Config".to_string()))?;
		config.value = value;
		Ok(())
	}

	pub fn update_with_path<P>(path: P, value: Value) -> Result<(), Error>
	where
		P: AsRef<Path>,
	{
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		map.values_mut()
			.filter(|config| config.path == path.as_ref())
			.for_each(|config| config.value = value.clone());
		Ok(())
	}

	pub fn all() -> Vec<ConfigEntry> {
		STORE.all()
	}
}
