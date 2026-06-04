//! # puniyu_adapter_core
//!
//! 统一的 puniyu 适配器核心库，覆盖适配器定义与注册表管理场景。

mod registry;
use puniyu_semver::Version;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_adapter_api::AdapterApi;
use puniyu_config::Config;
use puniyu_error::Result;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: AdapterApi + Send + Sync + 'static {
    fn core_version(&self) -> Version {
        puniyu_version::VERSION
    }

    fn config(&self) -> Vec<Arc<dyn Config>> {
        Vec::new()
    }

    fn server(&self) -> Option<puniyu_server::ServerFunction> {
        None
    }

    async fn on_load(&self) -> Result {
        Ok(())
    }

    async fn on_unload(&self) -> Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use puniyu_account::AccountInfo;
    use puniyu_adapter_types::{AdapterInfo, SendMsgType};
    use puniyu_contact::ContactType;
    use puniyu_message::Message;
    use std::time::Duration;

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

    #[tokio::test]
    async fn adapter_core_version_matches_puniyu_version() {
        let adapter = MockAdapter::new("test");
        assert_eq!(adapter.core_version(), puniyu_version::VERSION);
    }

    #[test]
    fn adapter_default_config_returns_empty() {
        let adapter = MockAdapter::new("test");
        let configs: Vec<Arc<dyn Config>> = adapter.config();
        assert!(configs.is_empty());
    }

    #[test]
    fn adapter_default_server_returns_none() {
        let adapter = MockAdapter::new("test");
        assert!(adapter.server().is_none());
    }

    #[tokio::test]
    async fn adapter_default_on_load_returns_ok() {
        let adapter = MockAdapter::new("test");
        assert!(adapter.on_load().await.is_ok());
    }

    #[tokio::test]
    async fn adapter_default_on_unload_returns_ok() {
        let adapter = MockAdapter::new("test");
        assert!(adapter.on_unload().await.is_ok());
    }
}
