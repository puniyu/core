mod store;

use crate::{TaskId, TaskInfo};
use crate::handle::TaskHandle;
use puniyu_error::registry::Error;
use std::sync::LazyLock;
use store::TaskStore;

static STORE: LazyLock<TaskStore> = LazyLock::new(TaskStore::new);

/// 初始化任务调度器
#[inline]
pub async fn init() {
	store::init_scheduler().await;
}

/// 任务注册表
///
/// 提供任务的注册、卸载和查询功能。
///
/// # 功能
///
/// - 注册任务到调度器
/// - 通过 ID、名称或插件 ID 卸载任务
/// - 查询任务信息
/// - 获取所有已注册任务
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_task::{Task, TaskRegistry};
/// use async_trait::async_trait;
/// use std::sync::Arc;
///
/// struct MyTask;
///
/// #[async_trait]
/// impl Task for MyTask {
///     fn name(&self) -> &'static str { "my_task" }
///     fn cron(&self) -> &'static str { "0 * * * * *" }
///     async fn run(&self) -> puniyu_error::Result { Ok(()) }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let plugin_id = 1;
///     let task = Arc::new(MyTask);
///
///     // 注册任务
///     let task_id = TaskRegistry::register(plugin_id, task).await.unwrap();
///
///     // 查询任务
///     let tasks = TaskRegistry::get(task_id);
///
///     // 卸载任务
///     TaskRegistry::unregister(task_id).await.unwrap();
/// }
/// ```
pub struct TaskRegistry;

impl<'t> TaskRegistry {
	/// 注册任务
	///
	/// 将任务添加到调度器并开始按照 Cron 表达式执行。
	///
	/// # 参数
	///
	/// - `plugin_id` - 关联的插件 ID
	/// - `handle` - 任务句柄
	///
	/// # 返回值
	///
	/// 返回分配给任务的唯一 ID。
	///
	/// # 错误
	///
	/// 如果任务注册失败，返回相应的错误信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_task::TaskRegistry;
	/// use std::sync::Arc;
	///
	/// let task_id = TaskRegistry::register(1, Arc::new(MyTask)).await?;
	/// println!("任务已注册，ID: {}", task_id);
	/// ```
	pub async fn register(plugin_id: u64, handle: TaskHandle) -> Result<u64, Error> {
		let task = TaskInfo { plugin_id, handle };
		STORE.insert(task).await
	}

	/// 卸载任务
	///
	/// 从调度器中移除任务，停止执行。
	///
	/// # 参数
	///
	/// - `task` - 任务 ID（`u64`）或任务名称（`&str`）
	///
	/// # 错误
	///
	/// 如果任务不存在或卸载失败，返回错误信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_task::TaskRegistry;
	///
	/// // 通过 ID 卸载
	/// TaskRegistry::unregister(123u64).await?;
	///
	/// // 通过名称卸载
	/// TaskRegistry::unregister("my_task").await?;
	/// ```
	pub async fn unregister<T>(task: T) -> Result<(), Error>
	where
		T: Into<TaskId<'t>>,
	{
		let task = task.into();
		match task {
			TaskId::Index(id) => Self::unregister_with_index(id).await,
			TaskId::Name(name) => Self::unregister_with_task_name(name.as_ref()).await,
		}
	}

	/// 通过索引卸载任务
	///
	/// # 参数
	///
	/// - `index` - 任务 ID
	///
	/// # 错误
	///
	/// 如果任务不存在，返回 `NotFound` 错误。
	pub async fn unregister_with_index(index: u64) -> Result<(), Error> {
		STORE.remove(index).await
	}

	/// 通过任务名称卸载任务
	///
	/// 如果有多个同名任务，将全部卸载。
	///
	/// # 参数
	///
	/// - `name` - 任务名称
	///
	/// # 错误
	///
	/// 如果找不到匹配的任务，返回 `NotFound` 错误。
	pub async fn unregister_with_task_name(name: &str) -> Result<(), Error> {
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter()
				.filter(|(_, task)| task.handle.get().name() == name)
				.map(|(id, _)| *id)
				.collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	/// 通过插件 ID 卸载所有关联任务
	///
	/// # 参数
	///
	/// - `plugin_id` - 插件 ID
	///
	/// # 错误
	///
	/// 如果找不到匹配的任务，返回 `NotFound` 错误。
	pub async fn unregister_with_plugin_id(plugin_id: u64) -> Result<(), Error> {
		let task_ids: Vec<u64> = {
			let raw = STORE.raw();
			let map = raw.read().expect("Failed to acquire lock");
			map.iter().filter(|(_, task)| task.plugin_id == plugin_id).map(|(id, _)| *id).collect()
		};

		if task_ids.is_empty() {
			return Err(Error::NotFound("Task".to_string()));
		}

		for id in task_ids {
			STORE.remove(id).await?;
		}
		Ok(())
	}

	/// 查询任务
	///
	/// 通过任务 ID 或名称查询任务信息。
	///
	/// # 参数
	///
	/// - `task` - 任务 ID（`u64`）或任务名称（`&str`）
	///
	/// # 返回值
	///
	/// 返回匹配的任务信息列表。如果使用 ID 查询，最多返回一个任务。
	pub fn get<T>(task: T) -> Vec<TaskInfo>
	where
		T: Into<TaskId<'t>>,
	{
		let task = task.into();
		match task {
			TaskId::Index(id) => Self::get_with_index(id).into_iter().collect(),
			TaskId::Name(name) => Self::get_with_task_name(name.as_ref()),
		}
	}

	/// 通过索引查询任务
	pub fn get_with_index(index: u64) -> Option<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.get(&index).cloned()
	}

	/// 通过任务名称查询任务
	pub fn get_with_task_name(name: &str) -> Vec<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.handle.get().name() == name).cloned().collect()
	}

	/// 通过插件 ID 查询任务
	pub fn get_with_plugin_id(plugin_id: u64) -> Vec<TaskInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");
		map.values().filter(|v| v.plugin_id == plugin_id).cloned().collect()
	}

	/// 获取所有任务
	pub fn all() -> Vec<TaskInfo> {
		STORE.all()
	}
}
