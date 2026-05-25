use puniyu_adapter_core::Adapter;
use puniyu_adapter_core::AdapterRegistry;
use puniyu_bot::{Bot, BotRegistry};
use puniyu_common::source::SourceType;
use puniyu_error::Result;
use puniyu_path::adapter::*;
use puniyu_version::VERSION;
use std::{io::Error as IoError, sync::Arc};
use tokio::fs::create_dir_all;

use crate::logger::core_warn;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) -> Result {
	let name = adapter.runtime().adapter_info().name.clone();
	let core_version = adapter.core_version();
	if core_version <= VERSION {
		core_warn!(
			"{} ({}): adapter core version is too low, please upgrade to {} or higher",
			name,
			core_version,
			VERSION
		);
		return Ok(());
	}

	init_dir(config_dir().join(&name), &name, "config").await?;
	init_dir(data_dir().join(&name), &name, "data").await?;
	init_dir(resource_dir().join(&name), &name, "resource").await?;
	init_dir(temp_dir().join(&name), &name, "temp").await?;

	adapter
		.on_load()
		.await
		.map_err(|e| IoError::other(format!("Failed to on_load adapter {}: {}", name, e)))?;
	super::config::init_config(&name, adapter.config()).await?;

	let index = AdapterRegistry::register(Arc::clone(&adapter))
		.unwrap_or_else(|e| panic!("Failed to register adapter {}: {}", name, e));
	let source = SourceType::Adapter(index);

	register_adapter_components(index, source, adapter.server()).await;

	let runtime = adapter.runtime();
	for account in adapter.accounts() {
		let bot = Arc::new(Bot::new(Arc::clone(&runtime), account));
		if let Err(e) = BotRegistry::register(bot) {
			log::error!("[{}] Failed to register bot: {}", name, e);
		}
	}

	Ok(())
}

async fn init_dir(path: std::path::PathBuf, adapter_name: &str, dir_kind: &str) -> Result {
	if !path.exists() {
		create_dir_all(&path).await.map_err(|e| {
			IoError::other(format!(
				"Failed to create {} dir for adapter {}: {}",
				dir_kind, adapter_name, e
			))
		})?;
	}
	Ok(())
}

async fn register_adapter_components(
	adapter_id: u64,
	source: SourceType,
	server: Option<puniyu_server::ServerFunction>,
) {
	if let Some(server) = server {
		super::server::init_server(source, server).unwrap_or_else(|e| {
			panic!("Failed to init server for adapter {}: {:?}", adapter_id, e)
		});
	}
}
