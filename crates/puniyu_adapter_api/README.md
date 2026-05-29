# puniyu_adapter_api

统一的适配器 API trait 定义，描述协议层核心接口。

## 特性

- 提供 `AdapterApi` 基础 API trait，定义消息发送与元信息访问
- 提供 `OneBotAdapterApi` OneBot 协议 API trait，支持私聊与群消息
- `OneBotAdapterApi` 通过 blanket impl 自动实现 `AdapterApi`
- 每个 API 实例自包含适配器信息（`adapter_info`）与账号信息（`account_info`）

## 快速开始

```rust
use async_trait::async_trait;
use puniyu_adapter_api::OneBotAdapterApi;
use puniyu_adapter_types::{AdapterInfo, SendMsgType};
use puniyu_account::AccountInfo;
use puniyu_message::Message;

struct MyOneBotApi {
    adapter_info: AdapterInfo,
    account_info: AccountInfo,
}

#[async_trait]
impl OneBotAdapterApi for MyOneBotApi {
    async fn send_private_msg(&self, user_id: u64, message: &Message) -> puniyu_error::Result<SendMsgType> {
        // 实现私聊消息发送
        todo!()
    }

    async fn send_group_msg(&self, group_id: u64, message: &Message) -> puniyu_error::Result<SendMsgType> {
        // 实现群消息发送
        todo!()
    }

    fn adapter_info(&self) -> AdapterInfo {
        self.adapter_info.clone()
    }

    fn account_info(&self) -> AccountInfo {
        self.account_info.clone()
    }
}
```

实现 `OneBotAdapterApi` 后自动获得 `AdapterApi` 实现，无需额外代码。
