# puniyu_error

错误类型库，提供配置、注册表等场景的统一错误定义。

## 特性

- 提供统一错误类型 `Error`
- 提供 `Result` 类型别名
- 覆盖配置与注册表等常见场景
- 基于 `thiserror` 实现
- 可通过 `registry` feature 接入注册表错误

## 快速开始

```rust
use puniyu_error::{Error, Result};

fn example() -> Result<()> {
    Err(Error::Config("missing field".into()))
}
```