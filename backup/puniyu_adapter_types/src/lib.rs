//! # puniyu_adapter_types
//!
//! 统一的 puniyu 适配器类型库，覆盖适配器信息、消息结果与群好友资料场景。
//!
//! ## 特性
//!
//! - 提供 [`AdapterInfo`] 和 `adapter_info!` 宏
//! - 提供 [`MessageType`]、[`SendMsgType`]、[`MessageInfo`]
//! - 提供群聊、成员、好友资料类型
//! - 提供平台、标准、协议与通信方式枚举
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_adapter_types::{adapter_info, AdapterPlatform, AdapterProtocol, MessageType};
//!
//! let info = adapter_info!(
//!     "console",
//!     AdapterPlatform::QQ,
//!     AdapterProtocol::Console
//! );
//!
//! assert_eq!(info.name, "console");
//!
//! let message = MessageType::from("12345");
//! match message {
//!     MessageType::Id(id) => assert_eq!(id, "12345"),
//!     MessageType::Seq(_) => unreachable!(),
//! }
//! ```

mod types;
#[doc(inline)]
pub use types::*;
