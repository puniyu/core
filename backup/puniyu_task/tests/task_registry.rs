#![cfg(feature = "registry")]

use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_task::{Task, TaskHandle, TaskRegistry, init};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

struct TestTask {
	name: &'static str,
}

impl TestTask {
	fn new(name: &'static str) -> Self {
		Self { name }
	}
}

#[async_trait]
impl Task for TestTask {
	fn name(&self) -> &'static str {
		self.name
	}

	fn cron(&self) -> &'static str {
		"0 * * * * *"
	}

	async fn execute(&self) -> Result {
		Ok(())
	}
}

fn unique_suffix() -> u128 {
	SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos())
}

fn leaked_name(prefix: &str, suffix: u128) -> &'static str {
	Box::leak(format!("{prefix}_{suffix}").into_boxed_str())
}

#[tokio::test]
async fn test_registry_lifecycle_and_error_paths() {
	init().await;

	let suffix = unique_suffix();
	let shared_name = leaked_name("registry_shared", suffix);
	let extra_name = leaked_name("registry_extra", suffix);
	let missing_name = leaked_name("registry_missing", suffix);

	let plugin_a = (suffix % 1_000_000) as u64 + 10_000;
	let plugin_b = plugin_a + 1;

	let id_a = TaskRegistry::register(plugin_a, TaskHandle::new(Arc::new(TestTask::new(shared_name))))
		.await
		.unwrap_or_else(|err| panic!("register plugin_a shared failed: {err}"));
	let id_b = TaskRegistry::register(plugin_b, TaskHandle::new(Arc::new(TestTask::new(shared_name))))
		.await
		.unwrap_or_else(|err| panic!("register plugin_b shared failed: {err}"));
	let id_extra =
		TaskRegistry::register(plugin_a, TaskHandle::new(Arc::new(TestTask::new(extra_name))))
			.await
			.unwrap_or_else(|err| panic!("register plugin_a extra failed: {err}"));

	let duplicate =
		TaskRegistry::register(plugin_a, TaskHandle::new(Arc::new(TestTask::new(shared_name)))).await;
	match duplicate {
		Ok(_) => panic!("duplicate task in same plugin should fail"),
		Err(err) => assert_eq!(err.to_string(), "exists: Task"),
	}

	let by_index = TaskRegistry::get(id_a);
	assert_eq!(by_index.len(), 1);
	assert_eq!(by_index[0].plugin_id, plugin_a);
	assert_eq!(by_index[0].handle.get().name(), shared_name);

	let by_name = TaskRegistry::get(shared_name);
	assert_eq!(by_name.len(), 2);
	assert!(by_name.iter().any(|task| task.plugin_id == plugin_a));
	assert!(by_name.iter().any(|task| task.plugin_id == plugin_b));

	let by_plugin_a = TaskRegistry::get_with_plugin_id(plugin_a);
	assert_eq!(by_plugin_a.len(), 2);
	assert!(by_plugin_a.iter().any(|task| task.handle.get().name() == shared_name));
	assert!(by_plugin_a.iter().any(|task| task.handle.get().name() == extra_name));

	TaskRegistry::unregister(id_extra)
		.await
		.unwrap_or_else(|err| panic!("unregister by index failed: {err}"));
	assert!(TaskRegistry::get(id_extra).is_empty());

	TaskRegistry::unregister(shared_name)
		.await
		.unwrap_or_else(|err| panic!("unregister by name failed: {err}"));
	assert!(TaskRegistry::get(shared_name).is_empty());
	assert!(TaskRegistry::get_with_plugin_id(plugin_b).is_empty());

	let missing_plugin = TaskRegistry::unregister_with_plugin_id(plugin_a).await;
	match missing_plugin {
		Ok(_) => panic!("missing plugin unregister should fail"),
		Err(err) => assert_eq!(err.to_string(), "not found: Task"),
	}

	let missing_by_name = TaskRegistry::unregister(missing_name).await;
	match missing_by_name {
		Ok(_) => panic!("missing name unregister should fail"),
		Err(err) => assert_eq!(err.to_string(), "not found: Task"),
	}

	let missing_by_index = TaskRegistry::unregister(id_b).await;
	match missing_by_index {
		Ok(_) => panic!("missing index unregister should fail"),
		Err(err) => assert_eq!(err.to_string(), "not found: Task"),
	}
}
