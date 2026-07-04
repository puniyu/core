//! # puniyu_element
//!
//! 消息元素类型库，统一定义接收与发送元素模型。
//!
//! ## 特性
//!
//! - 提供 `receive` / `send` 两套元素类型
//! - 提供统一抽象 `Element` 与 `ElementType`
//! - 支持序列化与反序列化

pub mod receive;
pub mod send;
mod types;
#[doc(inline)]
pub use types::{Element, ElementType};

macro_rules! codegen_reexport {
    ($($module:ident => $type:ident),*) => {
        $(
            mod $module;
			#[doc(inline)]
            pub use $module::$type;
        )*
    };
}
pub(crate) use codegen_reexport;
