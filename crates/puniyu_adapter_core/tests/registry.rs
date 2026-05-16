#![cfg(feature = "registry")]

use std::sync::{Arc, Mutex, MutexGuard};

use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{
	AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType, adapter_info,
};
use puniyu_contact::ContactType;
use puniyu_message::Message;
use puniyu_runtime::{AdapterProvider, AdapterRuntime, SendMessage};

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug)]
struct TestRuntime {
	info: AdapterInfo,
}

impl AdapterProvider for TestRuntime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.info
	}
}

#[async_trait::async_trait]
impl SendMessage for TestRuntime {
	async fn send_message(
		&self,
		_contact: &ContactType<'_>,
		_message: &Message,
	) -> puniyu_error::Result<SendMsgType> {
		Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
	}
}

struct TestAdapter {
	runtime: Arc<TestRuntime>,
}

impl TestAdapter {
	fn new() -> Self {
		Self {
			runtime: Arc::new(TestRuntime {
				info: adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console),
			}),
		}
	}
}

#[async_trait::async_trait]
impl Adapter for TestAdapter {
	fn runtime(&self) -> Arc<dyn AdapterRuntime> {
		self.runtime.clone()
	}
}

fn test_guard() -> MutexGuard<'static, ()> {
	TEST_LOCK.lock().expect("failed to acquire adapter registry test lock")
}

fn cleanup() {
	let _ = AdapterRegistry::unregister_with_adapter_name("console");
}

#[test]
fn register_returns_index_and_makes_adapter_queryable() {
	let _guard = test_guard();
	cleanup();

	let adapter: Arc<dyn Adapter> = Arc::new(TestAdapter::new());
	let index = AdapterRegistry::register(adapter).expect("failed to register adapter");

	let by_index = AdapterRegistry::get_with_index(index).expect("adapter should exist by index");
	assert_eq!(by_index.runtime().adapter_info().name, "console");

	let by_name = AdapterRegistry::get_with_adapter_name("console");
	assert_eq!(by_name.len(), 1);

	cleanup();
}

#[test]
fn duplicate_registration_returns_exists_error() {
	let _guard = test_guard();
	cleanup();

	let first: Arc<dyn Adapter> = Arc::new(TestAdapter::new());
	let second: Arc<dyn Adapter> = Arc::new(TestAdapter::new());

	AdapterRegistry::register(first).expect("first register should succeed");
	let err = AdapterRegistry::register(second).expect_err("duplicate register should fail");

	assert!(err.to_string().contains("exists"));

	cleanup();
}
