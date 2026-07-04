use puniyu_command_types::*;

#[test]
fn test_arg_string() {
	let arg = Arg::string("name");
	assert_eq!(arg.name.as_ref(), "name");
	assert_eq!(arg.arg_type, ArgType::String);
}

#[test]
fn test_arg_int() {
	let arg = Arg::int("count");
	assert_eq!(arg.arg_type, ArgType::Int);
}

#[test]
fn test_arg_float() {
	let arg = Arg::float("price");
	assert_eq!(arg.arg_type, ArgType::Float);
}

#[test]
fn test_arg_bool() {
	let arg = Arg::bool("flag");
	assert_eq!(arg.arg_type, ArgType::Bool);
}

#[test]
fn test_arg_required() {
	let arg = Arg::string("name").required();
	assert!(arg.required);
}

#[test]
fn test_arg_optional() {
	let arg = Arg::string("name").optional();
	assert!(!arg.required);
}

#[test]
fn test_arg_description() {
	let arg = Arg::string("name").description("用户名");
	assert_eq!(arg.description.as_deref(), Some("用户名"));
}

#[test]
fn test_arg_positional() {
	let arg = Arg::string("name").positional();
	assert_eq!(arg.mode, ArgMode::Positional);
}

#[test]
fn test_arg_named() {
	let arg = Arg::string("name").named();
	assert_eq!(arg.mode, ArgMode::Named);
}

#[test]
fn test_arg_value_string() {
	let val = ArgValue::from("hello");
	assert_eq!(val.as_str(), Some("hello"));
}

#[test]
fn test_arg_value_int() {
	let val = ArgValue::from(42i64);
	assert_eq!(val.as_int(), Some(42));
}

#[test]
fn test_arg_value_float() {
	let val = ArgValue::from(4.12f64);
	assert_eq!(val.as_float(), Some(4.12));
}

#[test]
fn test_arg_value_bool() {
	let val = ArgValue::from(true);
	assert_eq!(val.as_bool(), Some(true));
}

#[test]
fn test_arg_value_display() {
	let val = ArgValue::from("test");
	assert_eq!(val.to_string(), "test");
}

// Permission 测试
#[test]
fn test_permission_all() {
	let perm = Permission::All;
	assert_eq!(perm, Permission::All);
}

#[test]
fn test_permission_master() {
	let perm = Permission::Master;
	assert_eq!(perm, Permission::Master);
}

#[test]
fn test_permission_default() {
	let perm = Permission::default();
	assert_eq!(perm, Permission::All);
}

#[test]
fn test_command_action_done() {
	let action = CommandAction::Done;
	assert_eq!(action, CommandAction::Done);
}

#[test]
fn test_command_action_continue() {
	let action = CommandAction::Continue;
	assert_eq!(action, CommandAction::Continue);
}

#[test]
fn test_command_action_done_result() {
	let result = CommandAction::done();
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), CommandAction::Done);
}

#[test]
fn test_command_action_continue_result() {
	let result = CommandAction::r#continue();
	assert!(result.is_ok());
	assert_eq!(result.unwrap(), CommandAction::Continue);
}

#[test]
fn test_command_action_from_unit() {
	let action: CommandAction = ().into();
	assert_eq!(action, CommandAction::Done);
}
