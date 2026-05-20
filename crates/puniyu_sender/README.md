# puniyu_sender

发送者类型库，统一好友、群聊和频道发送者信息模型。

## 特性

- 提供统一发送者数据模型 `Sender`
- 覆盖好友（FriendSender）、群聊（GroupSender）和频道发送者
- 支持发送者信息查询
- 适合作为事件和上下文中的基础类型层

## 快速开始

```rust
use puniyu_sender::Sender;

fn process(sender: &Sender) {
    match sender {
        Sender::Friend(f) => { /* 处理好友消息 */ }
        Sender::Group(g) => { /* 处理群消息 */ }
        Sender::Guild(g) => { /* 处理频道消息 */ }
    }
}
```