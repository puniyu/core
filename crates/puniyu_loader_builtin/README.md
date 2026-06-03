# puniyu_loader_builtin

内置加载器，用于在编译期通过构建器模式注册 adapter 和 plugin。

## 特性

- 提供 `BuiltinLoader` 构建器：`new()` → `with_adapter()` → `with_plugin()`
- 实现 `Loader` trait，`discover()` 产出 `ComponentSet`
- 自动设置 `ComponentSource::Builtin`，priority=0
- 零外部依赖，纯编译期组装

## 快速开始

```rust
use puniyu_loader_builtin::BuiltinLoader;
use puniyu_core::App;

let app = App::builder()
    .with_loader(BuiltinLoader::new()
        .with_adapter(MyAdapter)
        .with_plugin(MyPlugin),
    )
    .build();

app.run().await?;
```