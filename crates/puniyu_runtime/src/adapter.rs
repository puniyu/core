use puniyu_adapter_types::AdapterInfo;

#[derive(Clone)]
pub struct AdapterRuntime {
    info: AdapterInfo,
}

impl AdapterRuntime {
    pub fn new(info: AdapterInfo) -> Self {
        Self { info }
    }

    pub fn info(&self) -> &AdapterInfo {
        &self.info
    }
}