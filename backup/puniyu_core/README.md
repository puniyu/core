# puniyu_core

应用装配与运行入口，通过 Loader 将 adapter 和 plugin 组织到统一的应用中，并执行 Discovery → Resolve → Install 三层启动流程。

## 特性

- 提供 `App` 和 `App::builder()` 作为主入口
- 所有组件（adapter/plugin）必须通过 Loader（如 `BuiltinLoader`）传入
- 启动时自动执行三层流程：发现 → 解析（过滤/去重/排序）→ 安装
- 支持设置应用名称、版本、Logo 和工作目录
- 支持注册处理器（handler）和顶层配置

## 快速开始

```rust
use puniyu_core::App;
use puniyu_loader_builtin::BuiltinLoader;

let app = App::builder()
    .with_app_name("my_bot")
    .with_loader(BuiltinLoader::new()
        .with_adapter(MyAdapter)
        .with_plugin(MyPlugin),
    )
    .build();

app.run().await?;
```