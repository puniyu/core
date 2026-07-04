# puniyu_task

轻量定时任务接口，基于 Cron 表达式定义任务执行时间。

## 特性

- 提供 `Task` trait 统一定义任务
- 支持 6 位标准 Cron：`秒 分 时 日 月 周`
- 提供 `TaskRegistry` 管理任务注册（需启用 `registry` feature）
- 提供 `init` 函数初始化任务调度

## 快速开始

```rust
use puniyu_task::{Task, init};
use async_trait::async_trait;
use puniyu_error::Result;

struct MyTask;

#[async_trait]
impl Task for MyTask {
    fn name(&self) -> &'static str { "my_task" }
    fn cron(&self) -> &'static str { "0 0 3 * * *" }
    async fn execute(&self) -> Result { Ok(()) }
}

// 初始化任务调度
init();
```