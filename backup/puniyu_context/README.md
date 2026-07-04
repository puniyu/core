# puniyu_context

统一的上下文类型，覆盖机器人、事件与消息处理场景。

## 特性

- 提供 `BotContext`：访问机器人实例、运行时、发送消息
- 提供 `EventContext`：访问事件信息、判断消息类型
- 提供 `MessageContext`：继承 `EventContext`，支持命令参数访问和快捷回复
- 提供 `EventArg` 类型别名（`HashMap<String, ArgValue>`）

## 快速开始

```rust
use puniyu_context::{BotContext, EventContext, MessageContext};

async fn handle_message(ctx: &MessageContext<'_>) {
    // 回复消息
    ctx.reply("Hello!").await.unwrap();

    // 获取命令参数并回复
    ctx.arg("name")
        .and_then(|arg| arg.as_str())
        .map(|name| ctx.reply(format!("Hello, {}!", name)));

    // 判断消息类型
    if ctx.is_group() {
        // 处理群消息
    }
}
```