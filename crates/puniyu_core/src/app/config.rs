use puniyu_config::{Config, ConfigRegistry};
use puniyu_error::Result;
use std::{io::Error, sync::Arc};

pub async fn init_config(name: &str, configs: Vec<Arc<dyn Config>>) -> Result {
	for config in configs {
		let config_name = config.name().to_string();
		let dir = puniyu_path::config_dir().join(name);
		let file_path = dir.join(format!("{}.toml", &config_name));
		ConfigRegistry::register_entry(&config_name, file_path, config.to_value()).map_err(|e| {
			Error::other(format!("Failed to register config {} for {}: {:?}", config_name, name, e))
		})?;
	}
	Ok(())
}
