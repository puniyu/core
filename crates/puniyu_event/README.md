# puniyu_event

事件类型库，统一消息、通知和请求等事件模型。

## 特性

- 提供统一事件模型 `Event`
- 覆盖消息（FriendMessage、GroupMessage）、通知、请求等事件类型
- 提供 `extension` 模块支持事件扩展字段
- 提供 `message` 子模块管理各类消息事件

## 快速开始

```rust
use puniyu_event::Event;

fn process(event: &Event<'_>) {
    match event {
        Event::Message(msg) => { /* 处理消息事件 */ }
        Event::Request(req) => { /* 处理请求事件 */ }
        Event::Notice(notice) => { /* 处理通知事件 */ }
        _ => {}
    }
}
```