//! # puniyu_context
//!
//! 统一的上下文类型，覆盖机器人、事件与消息处理场景。
//!
//! ## 概述
//!
//! `puniyu_context` 提供了统一的上下文管理系统，用于处理聊天机器人中的各种上下文信息。
//! 该库将上下文分为三个层次：
//!
//! - **机器人上下文（BotContext）** - 提供对机器人实例和运行时的访问
//! - **事件上下文（EventContext）** - 提供对事件信息和命令参数的访问
//! - **消息上下文（MessageContext）** - 提供对消息事件的专门处理
//!
//! ## 使用方式
//!
//! ### 创建机器人上下文
//!
//! ```rust,ignore
//! use puniyu_context::BotContext;
//!
//! let bot_context = BotContext::new(&bot);
//!
//! // 访问运行时
//! let runtime = bot_context.runtime();
//! // let concrete = runtime.downcast_ref::<MyRuntime>();
//!
//! // 发送消息
//! // bot_context.send_message(&contact, message).await?;
//!
//! // 访问账号信息
//! let account = bot_context.account();
//! ```
//!
//! ### 处理事件上下文
//!
//! ```rust,ignore
//! use puniyu_context::{BotContext, EventContext};
//!
//! let bot_context = BotContext::new(&bot);
//! let event_context = EventContext::new(&event, Some(args));
//!
//! // 判断事件类型
//! if event_context.is_message() {
//!     if let Some(msg_ctx) = event_context.as_message() {
//!         // 处理消息
//!     }
//! }
//! ```
//!
//! ### 处理消息上下文
//!
//! ```rust,ignore
//! use puniyu_context::MessageContext;
//!
//! async fn handle_message(ctx: &MessageContext<'_>) {
//!     // 回复消息
//!     ctx.reply("Hello!").await.unwrap();
//!
//!     // 获取命令参数
//!     if let Some(arg) = ctx.arg("name") {
//!         if let Some(name) = arg.as_str() {
//!             ctx.reply(format!("Hello, {}!", name)).await.unwrap();
//!         }
//!     }
//!
//!     // 判断消息类型（由 MessageBase trait 提供）
//!     if ctx.is_group() {
//!         // 处理群消息
//!     }
//! }
//! ```

mod bot;
#[doc(inline)]
pub use bot::BotContext;
mod event;
#[doc(inline)]
pub use event::*;

use puniyu_command_types::ArgValue;
use std::collections::HashMap;

/// 命令参数映射类型。
pub type EventArg = HashMap<String, ArgValue>;
