# puniyu_account

机器人账户信息类型库，描述账号 UIN、昵称与头像数据。

## 特性

- 提供 `AccountInfo` 结构体定义账户信息
- 提供 `account_info!` 宏快速构建
- 提供 `AccountInfoBuilder` 构建器（基于 bon）
- 支持 `serde` 序列化与反序列化

## 快速开始

```rust
use puniyu_account::{AccountInfo, account_info};
use bytes::Bytes;

let account = account_info!(
    uin: "123456789",
    name: "Puniyu",
    avatar: Bytes::from_static(b"avatar"),
);
```