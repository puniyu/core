use std::sync::Arc;

use puniyu_adapter_core::Adapter;
use puniyu_adapter_core::AdapterRegistry;
use puniyu_common::source::SourceType;
use puniyu_error::Result;
use puniyu_version::VERSION;

use crate::logger::core_warn;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) -> Result {
    let name = adapter.name().to_string();
    let core_version = adapter.core_version();
    if core_version > VERSION {
        core_warn!(
            "{}: adapter requires framework version >= {}, but current version is {}",
            name,
            core_version,
            VERSION
        );
        return Ok(());
    }

    let index =
        AdapterRegistry::register(Arc::clone(&adapter)).unwrap_or_else(|e| {
            panic!("Failed to register adapter {}: {}", name, e)
        });
    let source = SourceType::Adapter(index);

    register_adapter_components(index, source, adapter.server()).await;

    adapter.on_load().await.map_err(|e| {
        std::io::Error::other(format!("Failed to on_load adapter {}: {}", name, e))
    })?;

    Ok(())
}

async fn register_adapter_components(
    adapter_id: u64,
    source: SourceType,
    server: Option<puniyu_server::ServerFunction>,
) {
    if let Some(server) = server {
        super::server::init_server(source, server).unwrap_or_else(|e| {
            panic!("Failed to init server for adapter {}: {:?}", adapter_id, e)
        });
    }
}
