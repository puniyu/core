//! # puniyu_message
//!
//! 消息链封装库，统一组合 `Elements`。
//!
//! ## 特性
//!
//! - `Message` 封装消息元素链
//! - `message!` 宏快速构建
//! - 支持常见类型转换
//!
//! ```rust
//! use puniyu_message::message;
//! use puniyu_element::send::{AtElement, Elements, TextElement};
//!
//! let msg = message!(
//!     Elements::At(AtElement::new("123456")),
//!     Elements::Text(TextElement::new(" hello")),
//! );
//! ```

mod macros;

use puniyu_element::send::{Elements, TextElement};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// 消息链类型。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Elements>);

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			let segments: Vec<String> = self.0.iter().map(|s| format!("{:#}", s)).collect();
			write!(f, "{}", segments.join("\n"))
		} else {
			let segments: Vec<String> = self.0.iter().map(|s| s.to_string()).collect();
			write!(f, "{}", segments.join(""))
		}
	}
}

impl From<Vec<Elements>> for Message {
	fn from(message: Vec<Elements>) -> Self {
		Message(message)
	}
}

impl From<Elements> for Message {
	fn from(elements: Elements) -> Self {
		Message(vec![elements])
	}
}

impl From<Message> for Vec<Elements> {
	fn from(message: Message) -> Self {
		message.0
	}
}

impl From<&str> for Message {
	fn from(value: &str) -> Self {
		Self(vec![Elements::Text(TextElement::new(value))])
	}
}

impl<'m> From<Cow<'m, str>> for Message {
	fn from(value: Cow<'m, str>) -> Self {
		Self(vec![Elements::Text(TextElement::new(value.into_owned()))])
	}
}

impl From<String> for Message {
	fn from(value: String) -> Self {
		Self(vec![Elements::Text(TextElement::new(value))])
	}
}
