# puniyu_version

版本号管理库，封装 Puniyu 框架的版本信息。

## 特性

- 提供 `VERSION` 常量，定义框架当前版本
- 基于 `puniyu_semver` 实现版本表示
- 供其他 crate 获取框架版本信息

## 快速开始

```rust
use puniyu_version::VERSION;

println!("Puniyu version: {}", VERSION);
```