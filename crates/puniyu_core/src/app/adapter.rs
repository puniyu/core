use puniyu_adapter_core::Adapter;
use puniyu_adapter_core::AdapterRegistry;
use puniyu_common::source::SourceType;
use puniyu_error::Result;
use puniyu_path::adapter::*;
use std::{io::Error as IoError, sync::Arc};
use tokio::fs::create_dir_all;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) -> Result {
	let name = adapter.runtime().adapter_info().name.clone();
	let hooks = adapter.hook();
	let server = adapter.server();

	init_dir(config_dir().join(&name), &name, "config").await?;
	init_dir(data_dir().join(&name), &name, "data").await?;
	init_dir(resource_dir().join(&name), &name, "resource").await?;
	init_dir(temp_dir().join(&name), &name, "temp").await?;

	adapter
		.init()
		.await
		.map_err(|e| IoError::other(format!("Failed to init adapter {}: {}", name, e)))?;
	super::config::init_config(&name, adapter.config()).await?;

	let index = AdapterRegistry::register(Arc::clone(&adapter))
		.unwrap_or_else(|e| panic!("Failed to register adapter {}: {}", name, e));
	let source = SourceType::Adapter(index);

	register_adapter_components(index, source, hooks, server).await;

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
	hooks: Vec<Arc<dyn puniyu_hook::Hook>>,
	server: Option<puniyu_server::ServerFunction>,
) {
	if !hooks.is_empty() {
		super::hook::init_hook(source, hooks)
			.unwrap_or_else(|e| panic!("Failed to init hook for adapter {}: {:?}", adapter_id, e));
	}

	if let Some(server) = server {
		super::server::init_server(source, server).unwrap_or_else(|e| {
			panic!("Failed to init server for adapter {}: {:?}", adapter_id, e)
		});
	}
}
