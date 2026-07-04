# puniyu_bot

机器人实例库，统一机器人实例与注册表访问模型。

## 特性

- 提供 `Bot` 机器人实例类型
- 提供 `BotId` 标识符，支持按索引或 UIN 定位
- 提供 `BotRegistry` 管理多机器人实例
- 提供 `BotRegistry` 注册表访问

## 快速开始

```rust
use puniyu_bot::{Bot, BotId, BotRegistry};

let bot_id: BotId = 123u64.into();
let self_id: BotId = "123456".into();
```