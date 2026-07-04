mod error;
mod store;
#[doc(inline)]
pub use error::Error;

use puniyu_event::Event;
use store::DispatchStore;

/// 全局事件发射器，负责将事件分发到所有已注册的处理器。
pub struct EventEmitter;

impl EventEmitter {
	/// 启动事件发射器。
	///
	/// 必须在 Tokio 异步运行时中调用。启动后才能通过 [`EventEmitter::emit`] 发送事件。
	///
	/// # Errors
	///
	/// 若当前不在 Tokio 运行时中，返回 [`Error::MissingTokioRuntime`]。
	pub fn run() -> Result<(), Error> {
		DispatchStore::run()
	}

	/// 停止事件发射器。
	///
	/// 停止后调用 [`EventEmitter::emit`] 将返回 [`Error::NotRunning`]。
	pub fn stop() {
		DispatchStore::stop()
	}

	/// 返回事件发射器当前是否处于运行状态。
	pub fn is_running() -> bool {
		DispatchStore::is_running()
	}

	/// 将事件分发到所有已注册的处理器。
	///
	/// 处理器按 `priority()` 升序依次执行，单个处理器出错不会中断后续处理器。
	///
	/// # Errors
	///
	/// 若事件发射器未启动，返回 [`Error::NotRunning`]。
	pub async fn emit(event: Event<'_>) -> Result<(), Error> {
		if !DispatchStore::is_running() {
			return Err(Error::NotRunning);
		}
		let mut handles = puniyu_handler::HandlerRegistry::all();
		handles.sort_by_key(|h| h.priority());
		for handler in handles {
			if let Err(e) = handler.handle(&event).await {
				log::error!("[{}] handler error: {}", handler.name(), e);
			}
		}
		Ok(())
	}
}
