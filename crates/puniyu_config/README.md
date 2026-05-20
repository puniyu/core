# puniyu_config

统一的配置管理库，覆盖应用、Bot、群聊与好友场景。

## 特性

- 提供 `AppConfig`、`BotConfig`、`GroupConfig`、`FriendConfig`
- 提供 `app_config()`、`bot_config()`、`friend_config()`、`group_config()` 统一访问入口
- 提供 `ConfigRegistry` 管理已注册配置
- 提供 `init()` 函数初始化配置并启动监听

## 快速开始

```rust
use puniyu_config::{app_config, bot_config, init};

init();

let app = app_config();
let bot = bot_config().bot("bot_001");
```