# puniyu_handler

事件处理器库，提供统一的 `Handler` 接口。

## 特性

- 提供 `Handler` trait 定义事件处理模型
- 支持处理 `puniyu_event::Event`
- 支持优先级（值越小优先级越高）
- 提供 `HandlerRegistry` 管理处理器注册（需启用 `registry` feature）

## 快速开始

```rust
use puniyu_handler::Handler;
use puniyu_event::Event;
use puniyu_error::Result;

#[derive(Clone)]
struct MyHandler;

#[async_trait::async_trait]
impl Handler for MyHandler {
    fn name(&self) -> &'static str { "my_handler" }
    fn priority(&self) -> u32 { 5 }

    async fn handle(&self, event: &Event) -> Result {
        Ok(())
    }
}
```