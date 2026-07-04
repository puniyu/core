use std::path::PathBuf;

pub(crate) const NAME: &str = "plugins";

/// 获取插件配置目录，格式为 `{CONFIG_DIR}/plugins`。
pub fn config_dir() -> PathBuf {
	crate::config_dir().join(NAME)
}

/// 获取插件数据目录，格式为 `{DATA_DIR}/plugins`。
pub fn data_dir() -> PathBuf {
	crate::data_dir().join(NAME)
}

/// 获取插件资源目录，格式为 `{RESOURCE_DIR}/plugins`。
pub fn resource_dir() -> PathBuf {
	crate::resource_dir().join(NAME)
}

/// 获取插件临时目录，格式为 `{TEMP_DIR}/plugins`。
pub fn temp_dir() -> PathBuf {
	crate::temp_dir().join(NAME)
}
