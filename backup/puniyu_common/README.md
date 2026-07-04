# puniyu_common

通用能力库，提供应用信息等高频公共能力。

## 特性

- 提供 `app_name`、`app_version` 函数获取应用信息
- 提供 `app_cwd_dir` 获取工作目录
- 提供 `source` 模块定义来源类型
- 提供 `time` 模块处理时间相关功能

## 快速开始

```rust
use puniyu_common::app::{app_name, app_version, app_cwd_dir};

let name = app_name();
let version = app_version();
let cwd = app_cwd_dir();
```