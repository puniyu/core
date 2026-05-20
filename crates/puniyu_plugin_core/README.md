# puniyu_plugin_core

插件协议层，定义 Puniyu 中插件的核心抽象和可扩展能力。

## 特性

- 提供 `Plugin` trait，定义插件生命周期和扩展点
- 支持插件元信息（名称、版本、描述、作者）
- 支持命令、任务、配置、服务器扩展
- 提供 `PluginRegistry` 管理插件注册（需启用 `registry` feature）

## 快速开始

```rust
use puniyu_plugin_core::Plugin;
use puniyu_semver::Version;
use puniyu_error::Result;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }
    fn version(&self) -> Version { Version::new(1, 0, 0) }
    fn description(&self) -> Option<&str> { Some("my plugin") }
    async fn on_load(&self) -> Result { Ok(()) }
}
```