//! 任务类型测试

use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_task::{Task, TaskHandle, TaskId, TaskInfo};
use std::sync::Arc;

struct DummyTask;

#[async_trait]
impl Task for DummyTask {
	fn name(&self) -> &'static str {
		"dummy"
	}

	fn cron(&self) -> &'static str {
		"0 * * * * *"
	}

	async fn execute(&self) -> Result {
		Ok(())
	}
}

#[test]
fn test_task_info_creation() {
	let handle = TaskHandle::new(Arc::new(DummyTask));
	let task_info = TaskInfo { plugin_id: 1, handle };

	assert_eq!(task_info.plugin_id, 1);
	assert_eq!(task_info.handle.get().name(), "dummy");
}

#[test]
fn test_task_info_clone() {
	let handle = TaskHandle::new(Arc::new(DummyTask));
	let task_info = TaskInfo { plugin_id: 2, handle };

	let cloned = task_info.clone();
	assert_eq!(cloned.plugin_id, task_info.plugin_id);
	assert_eq!(cloned.handle.get().name(), task_info.handle.get().name());
}

#[test]
fn test_task_id_from_u64() {
	let task_id: TaskId = 123u64.into();
	match task_id {
		TaskId::Index(id) => assert_eq!(id, 123),
		_ => panic!("Expected TaskId::Index"),
	}
}

#[test]
fn test_task_id_from_str() {
	let task_id: TaskId = "test_task".into();
	match task_id {
		TaskId::Name(name) => assert_eq!(name, "test_task"),
		_ => panic!("Expected TaskId::Name"),
	}
}

#[test]
fn test_task_id_variants() {
	let id_variant = TaskId::Index(42);
	let name_variant = TaskId::Name(std::borrow::Cow::Borrowed("my_task"));

	match id_variant {
		TaskId::Index(id) => assert_eq!(id, 42),
		_ => panic!("Expected Index variant"),
	}

	match name_variant {
		TaskId::Name(name) => assert_eq!(name, "my_task"),
		_ => panic!("Expected Name variant"),
	}
}

#[test]
fn test_task_id_debug() {
	let id = TaskId::Index(100);
	let debug_str = format!("{:?}", id);
	assert!(debug_str.contains("Index"));
	assert!(debug_str.contains("100"));
}

#[test]
fn test_task_id_clone() {
	let id = TaskId::Index(999);
	let cloned = id.clone();

	match (id, cloned) {
		(TaskId::Index(a), TaskId::Index(b)) => assert_eq!(a, b),
		_ => panic!("Clone failed"),
	}
}

#[cfg(feature = "registry")]
#[test]
fn test_task_info_to_job_conversion() {
	use tokio_cron_scheduler::Job;

	let handle = TaskHandle::new(Arc::new(DummyTask));
	let task_info = TaskInfo { plugin_id: 1, handle };

	let job: Job = task_info.into();
	// 验证 Job 创建成功，guid 不为空
	let guid = job.guid();
	assert!(!guid.is_nil());
}
