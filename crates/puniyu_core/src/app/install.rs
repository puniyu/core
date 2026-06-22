use puniyu_common::source::SourceType;
use puniyu_loader::*;
use puniyu_version::VERSION;

use crate::app::resolve::ResolvedComponents;
use puniyu_common::{core_debug, core_error, core_info, core_warn};

pub(crate) async fn install(resolved: ResolvedComponents) -> std::io::Result<()> {
	core_debug!("adapter loading...");
	for adapter in resolved.adapters {
		if let Err(e) = install_adapter(adapter).await {
			core_error!("Failed to init adapter: {}", e);
		}
	}
	core_debug!("adapter loaded!");

	core_debug!("plugin loading...");
	for plugin in resolved.plugins {
		if let Err(e) = install_plugin(plugin).await {
			core_error!("Failed to init plugin: {}", e);
		}
	}
	core_debug!("plugin loaded!");

	core_info!("adapters: {}", puniyu_adapter_core::AdapterRegistry::all().len());
	core_info!("plugins: {}", puniyu_plugin_core::PluginRegistry::all().len());
	core_info!("commands: {}", puniyu_command::CommandRegistry::all().len());
	core_info!("handlers: {}", puniyu_handler::HandlerRegistry::all().len());
	Ok(())
}

async fn install_adapter(discovered: DiscoveredAdapter) -> puniyu_error::Result {
	let adapter = discovered.handle.get();
	let name = adapter.adapter_info().name.to_string();
	let core_version = adapter.core_version();

	if core_version > VERSION {
		core_warn!(
			"{}: adapter requires framework version >= {}, but current version is {}",
			name,
			core_version,
			VERSION
		);
		return Ok(());
	}

	let index = puniyu_adapter_core::AdapterRegistry::register(discovered.handle)
		.unwrap_or_else(|e| panic!("Failed to register adapter {}: {}", name, e));
	let source = SourceType::Adapter(index);

	if let Some(server) = adapter.server() {
		super::server::init_server(source, server)
			.unwrap_or_else(|e| panic!("Failed to register server for adapter {}: {:?}", name, e));
	}

	adapter
		.on_load()
		.await
		.map_err(|e| std::io::Error::other(format!("Failed to on_load adapter {}: {}", name, e)))?;

	Ok(())
}

async fn install_plugin(discovered: DiscoveredPlugin) -> puniyu_error::Result {
	let plugin = discovered.handle.get();
	let name = plugin.name().to_string();
	let core_version = plugin.core_version();

	if core_version > VERSION {
		core_warn!(
			"{}: plugin requires framework version >= {}, but current version is {}",
			name,
			core_version,
			VERSION
		);
		return Ok(());
	}
	init_plugin_dirs(&name).await?;
	super::config::init_config(&name, plugin.config()).await?;

	plugin.on_load().await.map_err(|e| {
		std::io::Error::other(format!("Failed to on_load plugin {}: {:?}", name, e))
	})?;

	let index = puniyu_plugin_core::PluginRegistry::register(discovered.handle)
		.unwrap_or_else(|e| panic!("Failed to register plugin {}: {}", name, e));
	let source = SourceType::Plugin(index);

	init_commands(index, plugin.commands())
		.unwrap_or_else(|e| panic!("Failed to register command for plugin {}: {:?}", name, e));

	init_tasks(index, plugin.tasks())
		.await
		.unwrap_or_else(|e| panic!("Failed to register task for plugin {}: {:?}", name, e));
	if let Some(server) = plugin.server() {
		super::server::init_server(source, server)
			.unwrap_or_else(|e| panic!("Failed to register server for plugin {}: {:?}", name, e));
	}

	Ok(())
}

async fn init_plugin_dirs(name: &str) -> puniyu_error::Result {
	use puniyu_path::plugin::*;
	use tokio::fs::create_dir_all;

	let dirs = vec![
		(config_dir().join(name), "config"),
		(data_dir().join(name), "data"),
		(resource_dir().join(name), "resource"),
		(temp_dir().join(name), "temp"),
	];

	for (path, dir_kind) in dirs {
		if !path.exists() {
			create_dir_all(&path).await.map_err(|e| {
				std::io::Error::other(format!(
					"Failed to create {} dir for plugin {}: {}",
					dir_kind, name, e
				))
			})?;
		}
	}

	Ok(())
}

fn init_commands(
	plugin_id: u64,
	commands: Vec<puniyu_command::CommandHandle>,
) -> std::result::Result<(), puniyu_error::registry::Error> {
	use puniyu_command::CommandRegistry;
	for command in commands {
		CommandRegistry::register(plugin_id, command)?;
	}
	Ok(())
}

async fn init_tasks(
	plugin_id: u64,
	tasks: Vec<puniyu_task::TaskHandle>,
) -> std::result::Result<(), puniyu_error::registry::Error> {
	use puniyu_task::TaskRegistry;
	for task in tasks {
		TaskRegistry::register(plugin_id, task).await?;
	}
	Ok(())
}
