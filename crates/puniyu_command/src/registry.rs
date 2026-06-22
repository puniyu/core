//! 命令注册表。

mod store;

use crate::handle::CommandHandle;
use crate::types::{CommandId, CommandInfo};
use puniyu_error::registry::Error;
use std::sync::LazyLock;
use store::CommandStore;

static STORE: LazyLock<CommandStore> = LazyLock::new(CommandStore::new);

/// 命令注册表。
pub struct CommandRegistry;

impl<'c> CommandRegistry {
	/// 注册命令，并返回内部索引。
	pub fn register(plugin_id: u64, handle: CommandHandle) -> Result<u64, Error> {
		let command = CommandInfo { plugin_id, handle };
		STORE.insert(command)
	}

	/// 按索引或名称卸载命令。
	pub fn unregister<C>(command: C) -> Result<(), Error>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::unregister_with_index(v),
			CommandId::Name(v) => Self::unregister_with_command_name(v.as_ref()),
		}
	}

	/// 通过索引卸载命令。
	pub fn unregister_with_index(command_id: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&command_id).is_none() {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.remove(&command_id);
		Ok(())
	}

	/// 通过名称卸载命令。
	pub fn unregister_with_command_name(name: &str) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.handle.get().name() == name) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.handle.get().name() != name);
		Ok(())
	}

	/// 通过插件 ID 卸载所有命令。
	pub fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if !map.values().any(|v| v.plugin_id == plugin_id) {
			return Err(Error::NotFound("Command".to_string()));
		}
		map.retain(|_, v| v.plugin_id != plugin_id);
		Ok(())
	}

	/// 按索引或名称查询命令。
	pub fn get<C>(command: C) -> Vec<CommandInfo>
	where
		C: Into<CommandId<'c>>,
	{
		let command = command.into();
		match command {
			CommandId::Id(v) => Self::get_with_command_id(v).into_iter().collect(),
			CommandId::Name(v) => Self::get_with_command_name(v.as_ref()),
		}
	}

	/// 通过索引查询命令。
	pub fn get_with_command_id(id: u64) -> Option<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&id).cloned()
	}

	/// 通过命令名称查询命令。
	pub fn get_with_command_name(name: &str) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.handle.get().name() == name).cloned().collect()
	}

	/// 通过命令别名查询命令。
	pub fn get_with_command_alias(alias: &str) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.handle.get().alias().contains(&alias)).cloned().collect()
	}

	/// 通过插件 ID 查询所有命令。
	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<CommandInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect()
	}

	/// 获取所有已注册命令。
	pub fn all() -> Vec<CommandInfo> {
		STORE.all()
	}
}
