use crate::Error;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime::Handle;

static RUNNING: OnceLock<AtomicBool> = OnceLock::new();

fn running() -> &'static AtomicBool {
	RUNNING.get_or_init(|| AtomicBool::new(false))
}

pub(crate) struct DispatchStore;

impl DispatchStore {
	pub(crate) fn run() -> Result<(), Error> {
		Handle::try_current().map_err(|_| Error::MissingTokioRuntime)?;
		running().store(true, Ordering::Release);
		Ok(())
	}

	pub(crate) fn stop() {
		running().store(false, Ordering::Release);
	}

	pub(crate) fn is_running() -> bool {
		running().load(Ordering::Acquire)
	}
}
