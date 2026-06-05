use std::sync::Arc;

use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;

#[derive(Clone)]
pub struct AdapterRuntime {
	adapter: Arc<dyn AdapterApi>,
}

impl AdapterRuntime {
	pub fn new(adapter: Arc<dyn AdapterApi>) -> Self {
		Self { adapter }
	}

	pub fn adapter(&self) -> &Arc<dyn AdapterApi> {
		&self.adapter
	}

	pub fn adapter_info(&self) -> AdapterInfo {
		self.adapter.adapter_info()
	}

	pub fn account_info(&self) -> AccountInfo {
		self.adapter.account_info()
	}
}
