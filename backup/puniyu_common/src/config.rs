use puniyu_error::config::Error;
use serde::{Deserialize, Serialize};
use std::{
	fs,
	path::{Path, PathBuf},
};
use toml::{Value, from_str, to_string_pretty};

fn build_config_path<P: AsRef<Path>>(path: P, name: &str) -> PathBuf {
	let path = path.as_ref();
	if name.ends_with(".toml") { path.join(name) } else { path.join(format!("{}.toml", name)) }
}

fn write_to_file<T: Serialize>(full_path: &Path, data: &T) -> Result<(), Error> {
	if let Some(parent) = full_path.parent() {
		fs::create_dir_all(parent)?;
	}
	fs::write(full_path, to_string_pretty(data)?)?;
	Ok(())
}

fn to_toml_value<T: Serialize>(config: &T) -> Result<Value, Error> {
	Ok(from_str(&to_string_pretty(config)?)?)
}

/// 读取配置文件
///
/// # 示例
///
/// ```rust, ignore
/// use serde::Deserialize;
/// use std::path::Path;
/// use puniyu_common::config::read_config;
///
/// #[derive(Deserialize)]
/// struct Config {
///     name: String,
///     port: u16,
/// }
///
/// let config: Config = read_config(Path::new("./config"), "app").unwrap();
/// ```
pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Error>
where
	D: for<'de> Deserialize<'de>,
{
	let full_path = build_config_path(path, name);
	let config_str = fs::read_to_string(&full_path)?;
	Ok(from_str(&config_str)?)
}
/// 写入配置文件
///
/// # 示例
///
/// ```rust, ignore
/// use serde::Serialize;
/// use std::path::Path;
/// use puniyu_common::config::write_config;
///
/// #[derive(Serialize)]
/// struct Config {
///     name: String,
///     port: u16,
/// }
///
/// let config = Config {
///     name: "MyApp".to_string(),
///     port: 8080,
/// };
///
/// write_config(Path::new("./config"), "app", &config).unwrap();
/// ```
pub fn write_config<C>(path: &Path, name: &str, config: &C) -> Result<(), Error>
where
	C: Serialize,
{
	write_to_file(&build_config_path(path, name), config)
}

/// 更新配置文件
///
/// # 示例
///
/// ```rust, ignore
/// use serde::{Deserialize, Serialize};
/// use std::path::Path;
/// use puniyu_common::config::update_config;
///
/// #[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// pub struct Config {
///     field1: String,
///     field2: u32,
/// }
///
/// update_config::<Config, _>(Path::new("config"), "app", |config| {
///     config.field1 = "new value".to_string();
///     config.field2 = 42;
/// }).unwrap();
/// ```
pub fn update_config<C, P>(path: P, name: &str, updater: impl FnOnce(&mut C)) -> Result<(), Error>
where
	P: AsRef<Path>,
	C: Serialize + for<'de> Deserialize<'de> + Default,
{
	let path = path.as_ref();
	let mut config = read_config(path, name).unwrap_or_default();
	updater(&mut config);
	write_to_file(&build_config_path(path, name), &config)
}

/// 合并配置文件
///
/// 将用户配置覆盖到默认配置上，对于表类型会递归合并
///
/// # 示例
///
/// ```rust, ignore
/// use serde::Serialize;
/// use std::path::Path;
/// use puniyu_common::config::merge_config;
///
/// #[derive(Serialize)]
/// struct DefaultConfig {
///     name: String,
///     port: u16,
///     debug: bool,
/// }
///
/// #[derive(Serialize)]
/// struct UserConfig {
///     port: u16,
/// }
///
/// let default = DefaultConfig {
///     name: "MyApp".to_string(),
///     port: 3000,
///     debug: false,
/// };
///
/// let user = UserConfig {
///     port: 8080,
/// };
///
/// merge_config(Path::new("./config"), "app", &default, &user).unwrap();
/// ```
pub fn merge_config<P, C1, C2>(
	path: P,
	name: &str,
	default_config: &C1,
	user_config: &C2,
) -> Result<(), Error>
where
	P: AsRef<Path>,
	C1: Serialize,
	C2: Serialize,
{
	let mut base_value = to_toml_value(default_config)?;
	let fill_value = to_toml_value(user_config)?;
	merge_toml_values(&mut base_value, fill_value);
	write_to_file(&build_config_path(path, name), &base_value)
}

pub fn merge_toml_values(base: &mut Value, fill: Value) {
	match (base, fill) {
		(Value::Table(base_table), Value::Table(fill_table)) => {
			for (key, fill_value) in fill_table {
				if let Some(base_value) = base_table.get_mut(&key) {
					merge_toml_values(base_value, fill_value);
				} else {
					base_table.insert(key, fill_value);
				}
			}
		}
		(base, fill) => *base = fill,
	}
}

/// 删除配置文件中的指定节点
///
/// 支持嵌套节点删除，使用点号分隔路径
///
/// # 示例
///
/// ```rust, ignore
/// use std::path::Path;
/// use puniyu_common::config::delete_config;
///
/// delete_config(Path::new("config"), "app", "debug").unwrap();
/// delete_config(Path::new("config"), "app", "database.host").unwrap();
/// delete_config(Path::new("config"), "app", "server.ssl.enabled").unwrap();
/// ```
pub fn delete_config<P: AsRef<Path>>(path: P, name: &str, node_path: &str) -> Result<(), Error> {
	let full_path = build_config_path(path, name);
	let mut toml_value: Value = from_str(&fs::read_to_string(&full_path)?)?;
	delete_toml_node(&mut toml_value, node_path);
	write_to_file(&full_path, &toml_value)
}

fn delete_toml_node(value: &mut Value, node_path: &str) {
	let keys: Vec<&str> = node_path.split('.').collect();
	if keys.is_empty() {
		return;
	}

	let target_table =
		keys.iter().take(keys.len() - 1).try_fold(value, |current, &key| match current {
			Value::Table(table) => table.get_mut(key),
			_ => None,
		});

	if let (Some(Value::Table(table)), Some(&last_key)) = (target_table, keys.last()) {
		table.remove(last_key);
	}
}
