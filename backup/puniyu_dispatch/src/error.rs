use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Missing Tokio runtime")]
	MissingTokioRuntime,
	#[error("Event emitter is not running")]
	NotRunning,
}
