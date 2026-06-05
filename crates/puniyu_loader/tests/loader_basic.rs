use async_trait::async_trait;
use std::sync::Arc;

use puniyu_loader::{ComponentSet, LoadContext, Loader};

struct TestLoader;

#[async_trait]
impl Loader for TestLoader {
	fn name(&self) -> &'static str {
		"test_loader"
	}

	async fn discover(&self, _ctx: &LoadContext) -> puniyu_error::Result<ComponentSet> {
		Ok(ComponentSet { adapters: vec![], plugins: vec![] })
	}
}

#[tokio::test]
async fn test_loader_name() {
	let loader = TestLoader;
	assert_eq!(loader.name(), "test_loader");
}

#[tokio::test]
async fn test_loader_discover_empty() {
	let loader = TestLoader;
	let ctx = LoadContext { app_name: "test", cwd_dir: std::env::current_dir().unwrap() };
	let result = loader.discover(&ctx).await;
	assert!(result.is_ok());
	let set = result.unwrap();
	assert!(set.adapters.is_empty());
	assert!(set.plugins.is_empty());
}

#[tokio::test]
async fn test_loader_trait_object() {
	let loader: Arc<dyn Loader> = Arc::new(TestLoader);
	assert_eq!(loader.name(), "test_loader");
	let ctx = LoadContext { app_name: "test", cwd_dir: std::env::current_dir().unwrap() };
	let set = loader.discover(&ctx).await.unwrap();
	assert!(set.adapters.is_empty());
	assert!(set.plugins.is_empty());
}

#[tokio::test]
async fn test_loader_discover_with_context() {
	let loader = TestLoader;
	let ctx = LoadContext { app_name: "my_app", cwd_dir: std::path::PathBuf::from("/tmp") };
	assert_eq!(ctx.app_name, "my_app");
	let set = loader.discover(&ctx).await.unwrap();
	assert!(set.adapters.is_empty());
	assert!(set.plugins.is_empty());
}
