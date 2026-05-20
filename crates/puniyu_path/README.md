# puniyu_path

路径管理库，统一提供应用、插件与适配器目录路径。

## 特性

- 提供基础目录路径函数（`app_cwd_dir`、`app_dir`、`config_dir` 等）
- 提供插件目录 `plugin_dir`
- 提供适配器目录 `adapter_dir`
- 所有路径基于 `puniyu_common::app` 当前应用信息生成

## 快速开始

```rust
use puniyu_path::{app_dir, plugin_dir, adapter_dir, config_dir, data_dir};

let app = app_dir();
let cfg = config_dir();
let data = data_dir();
```