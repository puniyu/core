use puniyu_adapter_api::AdapterHandle;
use puniyu_account::AccountInfo;
use puniyu_adapter_types::AdapterInfo;
use std::sync::Arc;

#[derive(Clone)]
pub struct AdapterRuntime {
	handle: AdapterHandle,
}

impl AdapterRuntime {
	pub fn new(handle: AdapterHandle) -> Self {
		Self { handle }
	}

	pub fn handle(&self) -> &AdapterHandle {
		&self.handle
	}

	pub fn adapter(&self) -> Arc<dyn puniyu_adapter_api::AdapterApi> {
		self.handle.get()
	}

	pub fn set(&self, adapter: Arc<dyn puniyu_adapter_api::AdapterApi>) -> Arc<dyn puniyu_adapter_api::AdapterApi> {
		self.handle.set(adapter)
	}

	pub fn adapter_info(&self) -> AdapterInfo {
		self.handle.get().adapter_info()
	}

	pub fn account_info(&self) -> AccountInfo {
		self.handle.get().account_info()
	}
}
