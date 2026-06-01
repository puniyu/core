use async_trait::async_trait;
use puniyu_command::{Arg, Command, CommandAction, CommandRegistry};
use puniyu_command_parser::CommandParser;
use std::sync::{Arc, Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand;

#[async_trait]
impl Command for TestCommand {
	fn name(&self) -> &'static str {
		"hello"
	}

	fn args(&self) -> Vec<Arg<'_>> {
		vec![Arg::string("name").named().required()]
	}

	async fn execute(
		&self,
		_ctx: &puniyu_context::MessageContext,
	) -> puniyu_error::Result<CommandAction> {
		CommandAction::done()
	}
}

fn test_guard() -> MutexGuard<'static, ()> {
	TEST_LOCK.lock().expect("failed to acquire parser test lock")
}

fn register_test_command() {
	let _ = CommandRegistry::unregister_with_command_name("hello");
	CommandRegistry::register(0, Arc::new(TestCommand)).expect("failed to register test command");
}

fn unregister_test_command() {
	let _ = CommandRegistry::unregister_with_command_name("hello");
}

#[test]
fn strips_bot_alias_before_parsing() {
	let _guard = test_guard();
	register_test_command();

	let parser = CommandParser::builder()
		.aliases(vec!["@bot".to_string(), "/bot".to_string()])
		.parse("@bot hello --name Alice")
		.expect("failed to parse command with bot alias");

	assert_eq!(parser.command_name(), "hello");
	assert_eq!(parser.get("name").and_then(|v| v.as_str()), Some("Alice"));

	unregister_test_command();
}

#[test]
fn strips_prefix_before_parsing() {
	let _guard = test_guard();
	register_test_command();

	let parser = CommandParser::builder()
		.prefix(vec!["!".to_string(), "/".to_string()])
		.parse("!hello --name Alice")
		.expect("failed to parse command with prefix");

	assert_eq!(parser.command_name(), "hello");
	assert_eq!(parser.get("name").and_then(|v| v.as_str()), Some("Alice"));

	unregister_test_command();
}

#[test]
fn keeps_original_input_when_no_alias_or_prefix_matches() {
	let _guard = test_guard();
	register_test_command();

	let parser = CommandParser::builder()
		.aliases(vec!["@bot".to_string()])
		.prefix(vec!["!".to_string()])
		.parse("hello --name Alice")
		.expect("failed to parse command without alias or prefix");

	assert_eq!(parser.command_name(), "hello");
	assert_eq!(parser.get("name").and_then(|v| v.as_str()), Some("Alice"));

	unregister_test_command();
}
