use puniyu_command::Command;
use puniyu_common::source::SourceType;
use puniyu_error::Result;
use puniyu_error::registry::Error;
use puniyu_path::plugin::*;
use puniyu_plugin_core::Plugin;
use puniyu_plugin_core::PluginRegistry;
use puniyu_task::Task;
use std::{io::Error as IoError, sync::Arc};
use tokio::fs::create_dir_all;

use crate::logger::core_warn;

pub async fn init_plugin(plugin: Arc<dyn Plugin>) -> Result {
	let name = plugin.name();
	let core_version = plugin.core_version();
	if core_version <= super::VERSION {
		core_warn!(
			"{} ({}): plugin core version is too low, please upgrade to {} or higher",
			name,
			core_version,
			super::VERSION
		);
		return Ok(());
	}
	let hooks = plugin.hooks();
	let commands = plugin.commands();
	let tasks = plugin.tasks();
	let server = plugin.server();

	init_dir(config_dir().join(name), name, "config").await?;
	init_dir(data_dir().join(name), name, "data").await?;
	init_dir(resource_dir().join(name), name, "resource").await?;
	init_dir(temp_dir().join(name), name, "temp").await?;

	plugin
		.init()
		.await
		.map_err(|e| IoError::other(format!("Failed to init plugin {}: {:?}", name, e)))?;

	super::config::init_config(name, plugin.config()).await?;

	let index = PluginRegistry::register(Arc::clone(&plugin))
		.unwrap_or_else(|e| panic!("Failed to register plugin {}: {:?}", name, e));
	let source = SourceType::Plugin(index);

	register_plugin_components(index, source, hooks, commands, tasks, server).await;

	Ok(())
}

async fn init_dir(path: std::path::PathBuf, plugin_name: &str, dir_kind: &str) -> Result {
	if !path.exists() {
		create_dir_all(&path).await.map_err(|e| {
			IoError::other(format!(
				"Failed to create {} dir for plugin {}: {}",
				dir_kind, plugin_name, e
			))
		})?;
	}
	Ok(())
}

async fn register_plugin_components(
	plugin_id: u64,
	source: SourceType,
	hooks: Vec<Arc<dyn puniyu_hook::Hook>>,
	commands: Vec<Arc<dyn Command>>,
	tasks: Vec<Arc<dyn Task>>,
	server: Option<puniyu_server::ServerFunction>,
) {
	if !hooks.is_empty() {
		super::hook::init_hook(source, hooks).unwrap_or_else(|e| {
			panic!("Failed to register hook for plugin {}: {:?}", plugin_id, e)
		});
	}

	if !commands.is_empty() {
		init_command(plugin_id, commands).unwrap_or_else(|e| {
			panic!("Failed to register command for plugin {}: {:?}", plugin_id, e)
		});
	}

	if !tasks.is_empty() {
		init_task(plugin_id, tasks).await.unwrap_or_else(|e| {
			panic!("Failed to register task for plugin {}: {:?}", plugin_id, e)
		});
	}

	if let Some(server) = server {
		super::server::init_server(source, server).unwrap_or_else(|e| {
			panic!("Failed to register server for plugin {}: {:?}", plugin_id, e)
		});
	}
}

fn init_command(plugin_id: u64, commands: Vec<Arc<dyn Command>>) -> std::result::Result<(), Error> {
	use puniyu_command::CommandRegistry;
	for command in commands {
		CommandRegistry::register(plugin_id, command)?;
	}
	Ok(())
}

async fn init_task(plugin_id: u64, tasks: Vec<Arc<dyn Task>>) -> std::result::Result<(), Error> {
	use puniyu_task::TaskRegistry;
	for task in tasks {
		TaskRegistry::register(plugin_id, task).await?;
	}
	Ok(())
}
