use crate::handle::TaskHandle;
#[cfg(feature = "registry")]
use log::{error, info};
#[cfg(feature = "registry")]
use puniyu_logger::owo_colors::OwoColorize;
use std::borrow::Cow;
#[cfg(feature = "registry")]
use std::time::Instant;
#[cfg(feature = "registry")]
use tokio_cron_scheduler::JobBuilder;
/// 任务信息
///
/// 包含任务的插件 ID 和任务构建器。
///
/// # 字段
///
/// - `plugin_id` - 关联的插件 ID
/// - `handle` - 任务句柄
#[derive(Clone)]
pub struct TaskInfo {
	/// 关联的插件 ID
	pub plugin_id: u64,
	/// 任务句柄
	pub handle: TaskHandle,
}

#[cfg(feature = "registry")]
impl From<&TaskInfo> for tokio_cron_scheduler::Job {
	fn from(task: &TaskInfo) -> Self {
		let builder = task.handle.get();
		let cron_str = builder.cron().to_string();
		JobBuilder::new()
			.with_timezone(chrono::Local)
			.with_cron_job_type()
			.with_schedule(&cron_str)
			.expect("Invalid cron schedule")
			.with_run_async(Box::new({
				let task_name = builder.name().to_string();
				let handle = task.handle.clone();
				move |_uuid, _lock| {
					let task_name = task_name.clone();
					let handle = handle.clone();
					Box::pin(async move {
						let tag_str = format!("[task:{}]", task_name);
						let tag = tag_str.fg_rgb::<255, 192, 203>();
						info!("{} 开始执行", tag);

						let start_time = Instant::now();
						let result = handle.get().execute().await;
						let duration = start_time.elapsed().as_millis();

						match result {
							Ok(_) => info!("{} 执行完成,耗时: {}ms", tag, duration),
							Err(e) => error!("{} 执行失败,耗时: {}ms, 错误: {}", tag, duration, e),
						}
					})
				}
			}))
			.build()
			.expect("Failed to create job")
	}
}

#[cfg(feature = "registry")]
impl From<TaskInfo> for tokio_cron_scheduler::Job {
	fn from(task: TaskInfo) -> Self {
		tokio_cron_scheduler::Job::from(&task)
	}
}

/// 任务 ID 枚举
///
/// 用于标识任务的两种方式：索引或名称。
///
/// # 变体
///
/// - `Index(u64)` - 使用数字索引标识任务
/// - `Name(&str)` - 使用任务名称标识任务
///
/// # 示例
///
/// ```rust
/// use puniyu_task::TaskId;
///
/// // 使用索引
/// let id1: TaskId = 123u64.into();
///
/// // 使用名称
/// let id2: TaskId = "my_task".into();
/// ```
#[derive(Debug, Clone)]
pub enum TaskId<'t> {
	/// 任务索引 ID
	Index(u64),
	/// 任务名称
	Name(Cow<'t, str>),
}

impl From<u64> for TaskId<'_> {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'t> From<&'t str> for TaskId<'t> {
	fn from(value: &'t str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for TaskId<'_> {
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}
