#![cfg(feature = "registry")]

use async_trait::async_trait;
use puniyu_command::{Command, CommandAction, CommandHandle, CommandRegistry};
use puniyu_context::MessageContext;
use std::sync::{Arc, Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());

struct HelloCommand;

#[async_trait]
impl Command for HelloCommand {
	fn name(&self) -> &str {
		"hello"
	}

	fn alias(&self) -> Vec<&str> {
		vec!["hi"]
	}

	async fn execute(&self, _ctx: &MessageContext) -> puniyu_error::Result<CommandAction> {
		CommandAction::done()
	}
}

fn test_guard() -> MutexGuard<'static, ()> {
	TEST_LOCK.lock().expect("failed to acquire command registry test lock")
}

fn cleanup() {
	let _ = CommandRegistry::unregister_with_command_name("hello");
}

#[test]
fn register_returns_index_and_makes_command_queryable() {
	let _guard = test_guard();
	cleanup();

	let index = CommandRegistry::register(7, CommandHandle::new(Arc::new(HelloCommand)))
		.expect("failed to register command");

	let by_id = CommandRegistry::get_with_command_id(index).expect("command should exist by id");
	assert_eq!(by_id.handle.get().name(), "hello");

	let by_name = CommandRegistry::get_with_command_name("hello");
	assert_eq!(by_name.len(), 1);

	cleanup();
}

#[test]
fn command_can_be_queried_by_alias() {
	let _guard = test_guard();
	cleanup();

	CommandRegistry::register(7, CommandHandle::new(Arc::new(HelloCommand)))
		.expect("failed to register command");

	let by_alias = CommandRegistry::get_with_command_alias("hi");
	assert_eq!(by_alias.len(), 1);
	assert_eq!(by_alias[0].handle.get().name(), "hello");

	cleanup();
}
