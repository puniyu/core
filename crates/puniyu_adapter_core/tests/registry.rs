use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_contact::ContactType;
use puniyu_message::Message;
use std::sync::Arc;
use std::time::Duration;

/// 用于集成测试的模拟适配器。
struct MockAdapter {
    info: AdapterInfo,
    account: AccountInfo,
}

impl MockAdapter {
    fn new(name: &str) -> Self {
        Self {
            info: AdapterInfo::builder().name(name).build(),
            account: AccountInfo::builder()
                .uin("0")
                .name("test")
                .avatar(Bytes::new())
                .build(),
        }
    }
}

#[async_trait::async_trait]
impl AdapterApi for MockAdapter {
    async fn send_message(
        &self,
        _contact: &ContactType<'_>,
        _message: &Message,
    ) -> puniyu_error::Result<SendMsgType> {
        Ok(SendMsgType {
            message_id: "0".into(),
            time: Duration::from_secs(0),
        })
    }

    fn adapter_info(&self) -> AdapterInfo {
        self.info.clone()
    }

    fn account_info(&self) -> AccountInfo {
        self.account.clone()
    }
}

impl Adapter for MockAdapter {}

/// 所有 AdapterRegistry 测试放在一个函数中顺序执行，
/// 避免全局 STORE 在并行测试中出现竞态。
#[test]
fn adapter_registry_full_lifecycle() {
    // ---- register ----
    let adapter = Arc::new(MockAdapter::new("integration_test"));
    let index = AdapterRegistry::register(adapter).expect("register should succeed");

    // ---- register duplicate name ----
    let dup = Arc::new(MockAdapter::new("integration_test"));
    let result = AdapterRegistry::register(dup);
    assert!(result.is_err(), "重复名称注册应失败");

    // ---- get by index ----
    let found = AdapterRegistry::get(index);
    assert!(found.is_some(), "按索引查询应找到适配器");
    assert_eq!(found.unwrap().adapter_info().name, "integration_test");

    // ---- get by name ----
    let found = AdapterRegistry::get("integration_test");
    assert!(found.is_some(), "按名称查询应找到适配器");
    assert_eq!(found.unwrap().adapter_info().name, "integration_test");

    // ---- get not found ----
    let not_found = AdapterRegistry::get("nonexistent");
    assert!(not_found.is_none(), "不存在的适配器应返回 None");

    // ---- all contains registered ----
    let all = AdapterRegistry::all();
    let names: Vec<String> = all.iter().map(|a| a.adapter_info().name.clone()).collect();
    assert!(names.contains(&"integration_test".to_string()), "all() 应包含已注册的适配器");

    // ---- unregister by name ----
    let unreg_result = AdapterRegistry::unregister("integration_test");
    assert!(unreg_result.is_ok(), "按名称卸载应成功");

    // ---- verify unregistered ----
    let after_unreg = AdapterRegistry::get("integration_test");
    assert!(after_unreg.is_none(), "卸载后按名称查询应返回 None");

    // ---- unregister non-existent ----
    let err = AdapterRegistry::unregister("nonexistent");
    assert!(err.is_err(), "卸载不存在的适配器应报错");
}

#[test]
fn adapter_registry_register_and_unregister_by_index() {
    let adapter = Arc::new(MockAdapter::new("index_test"));
    let index = AdapterRegistry::register(adapter).expect("register should succeed");

    // 确认已注册
    assert!(AdapterRegistry::get(index).is_some());

    // 按索引卸载
    let result = AdapterRegistry::unregister(index);
    assert!(result.is_ok(), "按索引卸载应成功");

    // 确认已卸载
    assert!(AdapterRegistry::get(index).is_none());
}

#[test]
fn adapter_registry_unregister_nonexistent_index() {
    let result = AdapterRegistry::unregister(999999_u64);
    assert!(result.is_err(), "卸载不存在的索引应报错");
}

#[test]
fn adapter_registry_get_by_nonexistent_index() {
    let result = AdapterRegistry::get(999999_u64);
    assert!(result.is_none(), "查询不存在的索引应返回 None");
}

#[test]
fn adapter_registry_all_contains_only_registered() {
    let adapter = Arc::new(MockAdapter::new("all_test_unique"));
    AdapterRegistry::register(adapter).expect("register should succeed");

    let all = AdapterRegistry::all();
    let names: Vec<String> = all.iter().map(|a| a.adapter_info().name.clone()).collect();
    assert!(names.contains(&"all_test_unique".to_string()), "all() 应包含刚注册的适配器");

    // 清理
    AdapterRegistry::unregister("all_test_unique").ok();
}
