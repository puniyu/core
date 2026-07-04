use crate::{ServerFunction, ServerId, ServerInfo};
use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use std::sync::LazyLock;
mod store;
use store::ServerStore;

static STORE: LazyLock<ServerStore> = LazyLock::new(ServerStore::new);

pub struct ServerRegistry;

impl ServerRegistry {
	pub fn register(source: SourceType, server: impl Into<ServerFunction>) -> Result<u64, Error> {
		STORE.insert(ServerInfo { source, builder: server.into() })
	}

	pub fn get(source: SourceType) -> Option<ServerInfo> {
		STORE.find(|s| s.source == source)
	}

	pub fn all() -> Vec<ServerInfo> {
		STORE.all()
	}

	pub fn unregister(server: impl Into<ServerId>) -> Result<(), Error> {
		STORE.remove(server)
	}
}
