# puniyu_adapter_core

适配器核心库，统一适配器定义与注册表管理。

## 特性

- 提供 `Adapter` trait，定义适配器核心接口
- 支持 `AdapterRegistry` 注册表管理
- 提供 `AdapterId` 标识符类型
- 支持生命周期回调（`on_load` / `on_unload`）
- 支持配置列表与服务器扩展

## 快速开始

```rust
use puniyu_adapter_core::Adapter;
use puniyu_runtime::AdapterRuntime;

#[async_trait::async_trait]
impl Adapter for MyAdapter {
    fn runtime(&self) -> Arc<dyn AdapterRuntime> { /* ... */ }
    fn core_version(&self) -> Version { puniyu_version::VERSION }
    async fn on_load(&self) -> Result { Ok(()) }
}
```