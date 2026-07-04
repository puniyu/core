# puniyu_dispatch

事件分发库，提供全局事件发射和分发能力。

## 特性

- 提供 `EventEmitter` 全局事件发射器
- 支持启动/停止事件发射器
- 将事件分发到所有已注册的 Handler
- Handler 按优先级顺序执行

## 快速开始

```rust
use puniyu_dispatch::EventEmitter;
use puniyu_event::Event;

EventEmitter::run()?;

// 在事件发生时发射
EventEmitter::emit(event).await?;
```