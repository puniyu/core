use puniyu_account::AccountInfo;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;

use crate::AdapterRuntime;

#[derive(Clone)]
pub struct BotRuntime {
	adapter: AdapterRuntime,
}

impl BotRuntime {
	pub fn new(adapter: AdapterRuntime) -> Self {
		Self { adapter }
	}

	pub fn adapter_info(&self) -> AdapterInfo {
		self.adapter.adapter_info()
	}

	pub fn adapter_runtime(&self) -> &AdapterRuntime {
		&self.adapter
	}

	pub fn account_info(&self) -> AccountInfo {
		self.adapter.account_info()
	}

	pub fn api(&self) -> &dyn AdapterApi {
		self.adapter.adapter().as_ref()
	}
}
