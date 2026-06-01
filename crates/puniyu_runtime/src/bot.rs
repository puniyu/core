use std::sync::Arc;

use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;

use crate::AdapterRuntime;

#[derive(Clone)]
pub struct BotRuntime {
    adapter: AdapterRuntime,
    api: Arc<dyn AdapterApi>,
}

impl BotRuntime {
    pub fn new(adapter: AdapterRuntime, api: Arc<dyn AdapterApi>) -> Self {
        Self { adapter, api }
    }

    pub fn adapter_info(&self) -> AdapterInfo {
        self.api.adapter_info()
    }

    pub fn adapter_runtime(&self) -> &AdapterRuntime {
        &self.adapter
    }

    pub fn account_info(&self) -> AccountInfo {
        self.api.account_info()
    }

    pub fn api(&self) -> &dyn AdapterApi {
        &*self.api
    }
}
