#![cfg(feature = "registry")]

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use async_trait::async_trait;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_message::Message;

static TEST_LOCK: Mutex<()> = Mutex::new(());

struct TestAdapter {
    info: AdapterInfo,
    account: AccountInfo,
}

impl TestAdapter {
    fn new(name: &str) -> Self {
        let info = AdapterInfo::builder()
            .name(name)
            .platform(AdapterPlatform::QQ)
            .protocol(AdapterProtocol::Console)
            .build();
        let account = AccountInfo {
            uin: format!("{name}_uin"),
            name: format!("{name}_account"),
            ..AccountInfo::default()
        };
        Self { info, account }
    }
}

#[async_trait]
impl AdapterApi for TestAdapter {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType { message_id: "test-message".to_string(), time: Duration::from_secs(1) })
    }

    fn adapter_info(&self) -> AdapterInfo {
        self.info.clone()
    }

    fn account_info(&self) -> AccountInfo {
        self.account.clone()
    }
}

impl Adapter for TestAdapter {}

fn test_guard() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().expect("failed to acquire adapter registry test lock")
}

fn cleanup(name: &str) {
    let _ = AdapterRegistry::unregister_with_adapter_name(name);
}

#[test]
fn register_returns_index_and_makes_adapter_queryable() {
    let _guard = test_guard();
    let name = "adapter_registry_register";
    cleanup(name);

    let adapter: Arc<dyn Adapter> = Arc::new(TestAdapter::new(name));
    let index = AdapterRegistry::register(adapter).expect("failed to register adapter");

    let by_index = AdapterRegistry::get_with_index(index).expect("adapter should exist by index");
    assert_eq!(by_index.adapter_info().name, name);

    let by_name = AdapterRegistry::get_with_adapter_name(name);
    assert_eq!(by_name.len(), 1);
    assert_eq!(by_name[0].account_info().uin, format!("{name}_uin"));

    cleanup(name);
}

#[test]
fn duplicate_registration_returns_exists_error() {
    let _guard = test_guard();
    let name = "adapter_registry_duplicate";
    cleanup(name);

    let first: Arc<dyn Adapter> = Arc::new(TestAdapter::new(name));
    let second: Arc<dyn Adapter> = Arc::new(TestAdapter::new(name));

    AdapterRegistry::register(first).expect("first register should succeed");
    let err = AdapterRegistry::register(second).expect_err("duplicate register should fail");

    assert!(err.to_string().contains("exists"));

    cleanup(name);
}

#[test]
fn unregister_accepts_index_and_name() {
    let _guard = test_guard();
    let index_name = "adapter_registry_unregister_index";
    let name_name = "adapter_registry_unregister_name";
    cleanup(index_name);
    cleanup(name_name);

    let index_adapter: Arc<dyn Adapter> = Arc::new(TestAdapter::new(index_name));
    let name_adapter: Arc<dyn Adapter> = Arc::new(TestAdapter::new(name_name));

    let index = AdapterRegistry::register(index_adapter).expect("index adapter register should succeed");
    AdapterRegistry::register(name_adapter).expect("name adapter register should succeed");

    AdapterRegistry::unregister(index).expect("unregister by index should succeed");
    assert!(AdapterRegistry::get_with_index(index).is_none());

    AdapterRegistry::unregister(name_name).expect("unregister by name should succeed");
    assert!(AdapterRegistry::get_with_adapter_name(name_name).is_empty());

    cleanup(index_name);
    cleanup(name_name);
}

#[test]
fn get_supports_index_name_and_all_queries() {
    let _guard = test_guard();
    let first_name = "adapter_registry_get_first";
    let second_name = "adapter_registry_get_second";
    cleanup(first_name);
    cleanup(second_name);

    let before = AdapterRegistry::all().len();

    let first: Arc<dyn Adapter> = Arc::new(TestAdapter::new(first_name));
    let second: Arc<dyn Adapter> = Arc::new(TestAdapter::new(second_name));

    let first_index = AdapterRegistry::register(first).expect("first register should succeed");
    let second_index = AdapterRegistry::register(second).expect("second register should succeed");

    let by_index = AdapterRegistry::get(first_index);
    assert_eq!(by_index.len(), 1);
    assert_eq!(by_index[0].adapter_info().name, first_name);

    let by_name = AdapterRegistry::get(second_name);
    assert_eq!(by_name.len(), 1);
    assert_eq!(by_name[0].adapter_info().name, second_name);

    let all = AdapterRegistry::all();
    assert_eq!(all.len(), before + 2);
    assert!(all.iter().any(|adapter| adapter.adapter_info().name == first_name));
    assert!(all.iter().any(|adapter| adapter.adapter_info().name == second_name));

    AdapterRegistry::unregister(first_index).expect("cleanup first adapter should succeed");
    AdapterRegistry::unregister(second_index).expect("cleanup second adapter should succeed");
}

#[test]
fn adapter_api_as_protocol_can_downcast_to_concrete_type() {
    let adapter = TestAdapter::new("adapter_registry_as_protocol");

    let protocol = (&adapter as &dyn AdapterApi)
        .as_protocol::<TestAdapter>()
        .expect("adapter should downcast to concrete test adapter");

    assert_eq!(protocol.adapter_info().name, "adapter_registry_as_protocol");
    assert_eq!(protocol.account_info().name, "adapter_registry_as_protocol_account");
}
