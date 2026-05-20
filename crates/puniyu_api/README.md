# puniyu_api

对外 API 聚合层，统一导出常用模块与公共入口。

## 特性

- 聚合导出 bot、config、context、event、message 等常用模块
- 提供公共应用信息入口（`app_name`、`app_version`、`pkg_name`、`pkg_version`）
- 导出高频公共依赖（tokio、toml、async_trait、inventory）
- 降低上层集成时的导入成本

## 快速开始

优先从这个 crate 导入常用类型与公共入口，减少分散依赖多个基础库。

```rust
use puniyu_api::{app_name, app_version, tokio, async_trait};

println!("App: {}", app_name!());
```