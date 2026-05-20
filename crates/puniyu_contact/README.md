# puniyu_contact

联系人类型库，统一好友和群聊联系人模型。

## 特性

- 提供 `FriendContact` 好友联系人
- 提供 `GroupContact` 群聊联系人
- 提供 `GuildContact` 频道联系人
- 支持 `ContactType` 枚举统一访问不同联系人

## 快速开始

```rust
use puniyu_contact::{FriendContact, GroupContact, ContactType};

let contact = ContactType::Group(GroupContact { /* ... */ });
```