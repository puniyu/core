# puniyu_loader

加载器 trait 定义及组件发现类型，作为 Puniyu 组件发现、解析和安装三层架构的第一层。

## 特性

- 提供 `Loader` trait，定义组件的发现接口
- 通过 `discover()` 产出 `ComponentSet`（含 adapter 和 plugin）
- 提供 `DiscoveredAdapter` / `DiscoveredPlugin` 包装类型，携带发现元信息
- 提供 `DiscoveryMeta`、`ComponentSource`、`LoadContext` 辅助类型
- Loader 是临时一次性的，不进入 registry

## 快速开始

```rust
use puniyu_loader::{Loader, LoadContext, ComponentSet};
use puniyu_error::Result;

struct MyLoader;

#[async_trait::async_trait]
impl Loader for MyLoader {
    fn name(&self) -> &'static str { "my_loader" }

    async fn discover(&self, ctx: &LoadContext) -> Result<ComponentSet> {
        Ok(ComponentSet {
            adapters: vec![],
            plugins: vec![],
        })
    }
}
```