# puniyu_runtime

运行时抽象库，定义适配器运行时与消息发送等核心能力。

## 特性

- 提供 `Runtime` 和 `AdapterRuntime` 核心 trait
- 定义消息发送接口 `Sender`
- 提供 `AdapterInfo` 运行时信息访问
- 适合作为适配器能力与动态扩展的运行时边界层

## 快速开始

```rust
use puniyu_runtime::{Runtime, AdapterRuntime, Sender};
use puniyu_contact::ContactType;
use puniyu_message::Message;

impl Sender for MyRuntime {
    async fn send(&self, contact: ContactType, message: &Message) -> Result<SendMsgType> {
        // 实现发送逻辑
    }
}
```