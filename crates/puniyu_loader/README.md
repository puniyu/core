# puniyu_loader

加载器类型定义库，提供插件加载器的类型系统和注册管理。

## 特性

- 提供 `Loader` trait 定义加载器接口
- 支持加载器注册表 `LoaderRegistry`（需启用 `registry` feature）
- 每个加载器可管理多个插件
- 支持插件生命周期管理

## 快速开始

```rust
use puniyu_loader::{Loader, LoaderRegistry};
use puniyu_plugin_core::Plugin;
use std::sync::Arc;

struct MyLoader;

#[async_trait::async_trait]
impl Loader for MyLoader {
    fn name(&self) -> &'static str { "my_loader" }
    async fn plugins(&self) -> Vec<Arc<dyn Plugin>> { Vec::new() }
}

let loader = Arc::new(MyLoader);
LoaderRegistry::register(loader);
```