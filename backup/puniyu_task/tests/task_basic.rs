//! 基础任务功能测试

use async_trait::async_trait;
use puniyu_error::Result;
use puniyu_task::Task;
use std::{io, sync::Arc};

// 测试任务实现
struct TestTask {
	name: &'static str,
	cron_expr: &'static str,
	should_fail: bool,
}

#[async_trait]
impl Task for TestTask {
	fn name(&self) -> &'static str {
		self.name
	}

	fn cron(&self) -> &'static str {
		self.cron_expr
	}

	async fn execute(&self) -> Result {
		if self.should_fail { Err(io::Error::other("task failed").into()) } else { Ok(()) }
	}
}

fn assert_task_impl<T: Task + Send + Sync + 'static>() {}

#[test]
fn test_task_creation() {
	assert_task_impl::<TestTask>();
	let task = TestTask { name: "test_task", cron_expr: "0 * * * * *", should_fail: false };

	assert_eq!(task.name(), "test_task");
	assert_eq!(task.cron(), "0 * * * * *");
}

#[test]
fn test_task_cron_expression() {
	let task = TestTask { name: "hourly_task", cron_expr: "0 0 * * * *", should_fail: false };

	assert_eq!(task.cron(), "0 0 * * * *");
}

#[tokio::test]
async fn test_task_execution() {
	let task = TestTask { name: "exec_task", cron_expr: "0 * * * * *", should_fail: false };

	let result = task.execute().await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_task_execution_error() {
	let task = TestTask { name: "failed_task", cron_expr: "0 * * * * *", should_fail: true };

	let result = task.execute().await;
	assert!(result.is_err());
	if let Err(err) = result {
		assert_eq!(err.to_string(), "task failed");
	}
}

#[test]
fn test_task_arc() {
	let task =
		Arc::new(TestTask { name: "arc_task", cron_expr: "0 * * * * *", should_fail: false });

	assert_eq!(task.name(), "arc_task");
}

#[test]
fn test_multiple_tasks() {
	let task1 = TestTask { name: "task1", cron_expr: "0 * * * * *", should_fail: false };

	let task2 = TestTask { name: "task2", cron_expr: "0 0 * * * *", should_fail: false };

	assert_ne!(task1.name(), task2.name());
}

#[test]
fn test_cron_patterns() {
	// 每分钟
	let task1 = TestTask { name: "every_minute", cron_expr: "0 * * * * *", should_fail: false };
	assert_eq!(task1.cron(), "0 * * * * *");

	// 每小时
	let task2 = TestTask { name: "every_hour", cron_expr: "0 0 * * * *", should_fail: false };
	assert_eq!(task2.cron(), "0 0 * * * *");

	// 每天
	let task3 = TestTask { name: "every_day", cron_expr: "0 0 0 * * *", should_fail: false };
	assert_eq!(task3.cron(), "0 0 0 * * *");
}

#[test]
fn test_task_trait_object() {
	let task: Arc<dyn Task> =
		Arc::new(TestTask { name: "trait_object", cron_expr: "0 * * * * *", should_fail: false });

	assert_eq!(task.name(), "trait_object");
}
