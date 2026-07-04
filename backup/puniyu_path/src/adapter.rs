use std::path::PathBuf;

pub(crate) const NAME: &str = "adapters";

/// 获取适配器配置目录，格式为 `{CONFIG_DIR}/adapters`。
pub fn config_dir() -> PathBuf {
	crate::config_dir().join(NAME)
}

/// 获取适配器数据目录，格式为 `{DATA_DIR}/adapters`。
pub fn data_dir() -> PathBuf {
	crate::data_dir().join(NAME)
}

/// 获取适配器资源目录，格式为 `{RESOURCE_DIR}/adapters`。
pub fn resource_dir() -> PathBuf {
	crate::resource_dir().join(NAME)
}

/// 获取适配器临时目录，格式为 `{TEMP_DIR}/adapters`。
pub fn temp_dir() -> PathBuf {
	crate::temp_dir().join(NAME)
}
