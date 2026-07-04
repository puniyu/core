# puniyu_adapter_types

适配器类型库，统一适配器信息、消息结果和资料模型。

## 特性

- 提供 `AdapterInfo` 适配器信息类型
- 提供 `MessageReceipt` 消息回执类型
- 提供 `GroupProfile`、`FriendProfile` 等资料模型
- 支持头像（Avatar）、频道（Guild）等扩展类型
- 定义 `SendMsgType` 消息发送类型枚举

## 快速开始

```rust
use puniyu_adapter_types::{AdapterInfo, MessageReceipt, SendMsgType};

let info = adapter_info!("MyAdapter", "1.0.0");
let receipt = MessageReceipt { /* ... */ };
```