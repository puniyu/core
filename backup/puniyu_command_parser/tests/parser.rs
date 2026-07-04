use async_trait::async_trait;
use puniyu_command::{Arg, Command, CommandAction, CommandHandle, CommandRegistry};
use puniyu_command_parser::{CommandParser, Error};
use std::sync::{Arc, Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());

// ── 测试用命令 ──────────────────────────────────────────

struct HelloCommand;

#[async_trait]
impl Command for HelloCommand {
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

struct CalcCommand;

#[async_trait]
impl Command for CalcCommand {
    fn name(&self) -> &'static str {
        "calc"
    }

    fn alias(&self) -> Vec<&str> {
        vec!["c"]
    }

    fn args(&self) -> Vec<Arg<'_>> {
        vec![
            Arg::int("a").positional().required(),
            Arg::int("b").positional().required(),
            Arg::bool("verbose").named(),
            Arg::string("mode").named(),
        ]
    }

    async fn execute(
        &self,
        _ctx: &puniyu_context::MessageContext,
    ) -> puniyu_error::Result<CommandAction> {
        CommandAction::done()
    }
}

// ── 辅助函数 ───────────────────────────────────────────

fn test_guard() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().expect("failed to acquire parser test lock")
}

fn register_commands() {
    let _ = CommandRegistry::unregister_with_command_name("hello");
    let _ = CommandRegistry::unregister_with_command_name("calc");
    CommandRegistry::register(0, CommandHandle::new(Arc::new(HelloCommand)))
        .expect("failed to register hello");
    CommandRegistry::register(0, CommandHandle::new(Arc::new(CalcCommand)))
        .expect("failed to register calc");
}

fn unregister_commands() {
    let _ = CommandRegistry::unregister_with_command_name("hello");
    let _ = CommandRegistry::unregister_with_command_name("calc");
}

// ── alias / prefix 剥离 ────────────────────────────────

#[test]
fn strips_bot_alias_before_parsing() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::builder()
        .aliases(vec!["@bot".to_string(), "/bot".to_string()])
        .build()
        .parse("@bot hello --name Alice")
        .expect("failed to parse with bot alias");

    assert_eq!(result.command_name(), "hello");
    assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("Alice"));

    unregister_commands();
}

#[test]
fn strips_prefix_before_parsing() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::builder()
        .prefix(vec!["!".to_string(), "/".to_string()])
        .build()
        .parse("!hello --name Alice")
        .expect("failed to parse with prefix");

    assert_eq!(result.command_name(), "hello");
    assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("Alice"));

    unregister_commands();
}

#[test]
fn strips_alias_and_prefix_together() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::builder()
        .aliases(vec!["@bot".to_string()])
        .prefix(vec!["!".to_string()])
        .build()
        .parse("@bot !hello --name Bob")
        .expect("failed to parse with alias + prefix");

    assert_eq!(result.command_name(), "hello");
    assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("Bob"));

    unregister_commands();
}

#[test]
fn keeps_original_input_when_no_alias_or_prefix_matches() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::builder()
        .aliases(vec!["@bot".to_string()])
        .prefix(vec!["!".to_string()])
        .build()
        .parse("hello --name Alice")
        .expect("failed to parse without alias or prefix");

    assert_eq!(result.command_name(), "hello");
    assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("Alice"));

    unregister_commands();
}

// ── 命令别名查找 ───────────────────────────────────────

#[test]
fn resolves_command_by_alias() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("c 1 2").expect("failed to parse via alias");

    assert_eq!(result.command_name(), "c");
    assert_eq!(result.get("a").and_then(|v| v.as_int()), Some(1));
    assert_eq!(result.get("b").and_then(|v| v.as_int()), Some(2));

    unregister_commands();
}

// ── 位置参数 ───────────────────────────────────────────

#[test]
fn parses_positional_int_args() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("calc 10 20").expect("failed to parse positional args");

    assert_eq!(result.get("a").and_then(|v| v.as_int()), Some(10));
    assert_eq!(result.get("b").and_then(|v| v.as_int()), Some(20));

    unregister_commands();
}

// ── 命名参数 ───────────────────────────────────────────

#[test]
fn parses_named_string_arg() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("hello --name Charlie")
        .expect("failed to parse named arg");

    assert_eq!(result.get("name").and_then(|v| v.as_str()), Some("Charlie"));

    unregister_commands();
}

// ── Bool 参数 ──────────────────────────────────────────

#[test]
fn parses_bool_flag_without_value() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("calc 1 2 --verbose")
        .expect("failed to parse bool flag");

    assert_eq!(result.get("verbose").and_then(|v| v.as_bool()), Some(true));

    unregister_commands();
}

#[test]
fn parses_bool_flag_with_value() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("calc 1 2 --verbose false")
        .expect("failed to parse bool with value");

    assert_eq!(result.get("verbose").and_then(|v| v.as_bool()), Some(false));

    unregister_commands();
}

// ── 混合参数 ───────────────────────────────────────────

#[test]
fn parses_mixed_positional_and_named() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("calc 5 3 --verbose --mode fast")
        .expect("failed to parse mixed args");

    assert_eq!(result.get("a").and_then(|v| v.as_int()), Some(5));
    assert_eq!(result.get("b").and_then(|v| v.as_int()), Some(3));
    assert_eq!(result.get("verbose").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(result.get("mode").and_then(|v| v.as_str()), Some("fast"));

    unregister_commands();
}

// ── 引号处理 ───────────────────────────────────────────

#[test]
fn handles_quoted_values() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("hello --name \"Alice in Wonderland\"")
        .expect("failed to parse quoted value");

    assert_eq!(
        result.get("name").and_then(|v| v.as_str()),
        Some("Alice in Wonderland")
    );

    unregister_commands();
}

// ── 错误处理 ───────────────────────────────────────────

#[test]
fn errors_on_empty_input() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("");
    assert!(matches!(result, Err(Error::EmptyInput)));

    unregister_commands();
}

#[test]
fn errors_on_unknown_command() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("nonexistent --arg val");
    assert!(matches!(result, Err(Error::UnknownCommand { .. })));

    unregister_commands();
}

#[test]
fn errors_on_missing_required_arg() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("hello");
    assert!(matches!(result, Err(Error::MissingRequired { .. })));

    unregister_commands();
}

#[test]
fn errors_on_invalid_int_value() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("calc abc 2");
    assert!(matches!(result, Err(Error::InvalidValue { .. })));

    unregister_commands();
}

#[test]
fn errors_on_unknown_named_arg() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new().parse("hello --name Alice --unknown val");
    assert!(matches!(result, Err(Error::UnknownArgument { .. })));

    unregister_commands();
}

// ── ParseResult API ────────────────────────────────────

#[test]
fn api_contains_keys_len() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("calc 1 2 --mode fast")
        .expect("failed to parse");

    assert!(result.contains("a"));
    assert!(result.contains("b"));
    assert!(result.contains("mode"));
    assert!(!result.contains("verbose"));
    assert_eq!(result.len(), 3);

    let keys: Vec<&String> = result.keys().collect();
    assert_eq!(keys.len(), 3);

    unregister_commands();
}

#[test]
fn into_inner_returns_map() {
    let _guard = test_guard();
    register_commands();

    let result = CommandParser::new()
        .parse("hello --name Test")
        .expect("failed to parse");

    let map = result.into_inner();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("name").and_then(|v| v.as_str()), Some("Test"));

    unregister_commands();
}
