# puniyu_core

应用装配与运行入口，将插件、适配器、处理器和加载器组织到同一个应用中。

## 特性

- 提供 `App` 和 `App::builder()` 作为主入口
- 支持装配插件、适配器、处理器和加载器
- 支持设置应用名称、Logo 和工作目录
- 统一应用初始化与运行流程

## 快速开始

```rust
use puniyu_core::App;

let app = App::builder()
    .with_plugin(MyPlugin)
    .with_adapter(MyAdapter)
    .build();

app.run().await?;
```