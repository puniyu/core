use puniyu_loader::Loader;
use puniyu_loader_builtin::BuiltinLoader;

#[test]
fn test_builtin_loader_default() {
	let loader = BuiltinLoader::default();
	assert_eq!(loader.name(), "builtin");
}

#[test]
fn test_builtin_loader_name() {
	let loader = BuiltinLoader::new();
	assert_eq!(loader.name(), "builtin");
}

#[test]
fn test_builtin_loader_new() {
	let loader = BuiltinLoader::new();
	assert_eq!(loader.name(), "builtin");
}
