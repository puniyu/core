#![cfg(feature = "registry")]

use std::sync::{Arc, Mutex, MutexGuard};

use async_trait::async_trait;
use puniyu_adapter_api::OneBotAdapterApi;
use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[allow(dead_code)]
struct TestOneBotApi {
    adapter_info: AdapterInfo,
}

#[async_trait]
impl OneBotAdapterApi for TestOneBotApi {
    async fn send_private_msg(
        &self,
        _user_id: u64,
        _message: &puniyu_message::Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
    }

    async fn send_group_msg(
        &self,
        _group_id: u64,
        _message: &puniyu_message::Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-msg".to_string(), time: std::time::Duration::ZERO })
    }

    fn adapter_info(&self) -> AdapterInfo {
        self.adapter_info.clone()
    }

    fn account_info(&self) -> puniyu_account::AccountInfo {
        unimplemented!("not used in registry tests")
    }
}

#[allow(dead_code)]
struct TestAdapter {
    name: String,
    info: AdapterInfo,
}

impl TestAdapter {
    fn new() -> Self {
        let info = AdapterInfo::builder()
            .name("console")
            .platform(AdapterPlatform::QQ)
            .protocol(AdapterProtocol::Console)
            .build();
        let name = info.name.clone();
        Self { name, info }
    }
}

#[async_trait]
impl Adapter for TestAdapter {
    fn name(&self) -> &str {
        &self.name
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
    assert_eq!(by_index.name(), "console");

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