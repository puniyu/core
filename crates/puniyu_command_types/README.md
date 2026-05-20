# puniyu_command_types

命令类型库，定义命令参数、权限和执行动作等核心模型。

## 特性

- 提供命令参数 `Arg` 构建器
- 提供 `ArgValue` 参数值类型（字符串、整数、浮点、布尔）
- 提供权限级别 `Permission` 枚举
- 提供执行动作 `CommandAction` 类型

## 快速开始

```rust
use puniyu_command_types::{Arg, ArgValue, Permission, CommandAction};

let arg = Arg::string("name").required().min_len(1).max_len(50);
let action = CommandAction::done();
```