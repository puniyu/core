use crate::TaskInfo;
use puniyu_error::registry::Error;
use std::{
	collections::HashMap,
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
use tokio::sync::OnceCell;
use tokio_cron_scheduler::JobScheduler;
use uuid::Uuid;

static TASK_ID: AtomicU64 = AtomicU64::new(0);
static SCHEDULER: OnceCell<Arc<JobScheduler>> = OnceCell::const_new();

pub(crate) async fn init_scheduler() {
	let _ = SCHEDULER
		.get_or_init(|| async {
			let sched = JobScheduler::new().await.expect("Failed to create scheduler");
			sched.start().await.expect("Failed to start scheduler");
			Arc::new(sched)
		})
		.await;
}

#[derive(Default)]
pub(crate) struct TaskStore {
	tasks: Arc<RwLock<HashMap<u64, TaskInfo>>>,
	uuid_map: Arc<RwLock<HashMap<u64, Uuid>>>,
}

impl TaskStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn get_scheduler(&self) -> Result<&'static Arc<JobScheduler>, Error> {
		SCHEDULER.get().ok_or_else(|| Error::NotFound("Scheduler".to_string()))
	}

	pub async fn insert(&self, task: TaskInfo) -> Result<u64, Error> {
		let index = TASK_ID.fetch_add(1, Ordering::Relaxed);
		{
			let map = self.tasks.read().expect("Failed to acquire read lock");
			if map
				.values()
				.any(|v| v.plugin_id == task.plugin_id && v.handle.get().name() == task.handle.get().name())
			{
				return Err(Error::Exists("Task".to_string()));
			}
		}

		let job = tokio_cron_scheduler::Job::from(&task);
		let scheduler = self.get_scheduler()?;
		let uuid = scheduler.add(job).await.map_err(|e| Error::NotFound(e.to_string()))?;

		{
			let mut map = self.tasks.write().expect("Failed to acquire write lock");
			let mut uuid_map = self.uuid_map.write().expect("Failed to acquire write lock");

			uuid_map.insert(index, uuid);
			map.insert(index, task);
		}

		Ok(index)
	}

	pub async fn remove(&self, index: u64) -> Result<(), Error> {
		let uuid = {
			let mut uuid_map = self.uuid_map.write().expect("Failed to acquire lock");
			uuid_map.remove(&index)
		};

		if uuid.is_none() {
			return Err(Error::NotFound("Task".to_string()));
		}

		#[allow(clippy::unwrap_used)]
		let uuid = uuid.unwrap();

		let scheduler = self.get_scheduler()?;
		if scheduler.remove(&uuid).await.is_err() {
			return Err(Error::NotFound("Task".to_string()));
		}

		let mut tasks = self.tasks.write().expect("Failed to acquire lock");
		tasks.remove(&index);
		Ok(())
	}

	pub fn all(&self) -> Vec<TaskInfo> {
		let map = self.tasks.read().expect("Failed to acquire lock");
		map.values().cloned().collect()
	}

	pub fn raw(&self) -> Arc<RwLock<HashMap<u64, TaskInfo>>> {
		self.tasks.clone()
	}
}
