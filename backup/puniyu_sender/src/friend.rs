//! 好友发送者模块
//!
//! 提供好友发送者的类型定义和构建宏。

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Sender, Sex};

/// 好友发送者信息。
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 's"))]
pub struct FriendSender<'s> {
	/// 发送者id
	#[builder(into)]
	#[serde(borrow)]
	user_id: Cow<'s, str>,
	/// 用户昵称
	#[builder(into)]
	#[serde(borrow)]
	nick: Option<Cow<'s, str>>,
	/// 性别
	#[builder(default)]
	sex: Sex,
	/// 年龄
	age: Option<u32>,
}

impl<'s> FriendSender<'s> {
	pub fn new<U, N>(user_id: U, nick: Option<N>, sex: Sex, age: Option<u32>) -> Self
	where
		U: Into<Cow<'s, str>>,
		N: Into<Cow<'s, str>>,
	{
		Self { user_id: user_id.into(), nick: nick.map(Into::into), sex, age }
	}
}

impl<'s> Sender for FriendSender<'s> {
	fn user_id(&self) -> &str {
		self.user_id.as_ref()
	}
	fn name(&self) -> Option<&str> {
		self.nick.as_deref()
	}
	fn sex(&self) -> &Sex {
		&self.sex
	}
	fn age(&self) -> Option<u32> {
		self.age
	}
}

/// 构建好友发送者。
///
/// ```rust
/// use puniyu_sender::{sender_friend, Sex};
///
/// let sender = sender_friend!(
///     user_id: "123456",
///     nick: "Alice",
///     sex: Sex::Female,
///     age: 25u32,
/// );
/// ```
#[macro_export]
macro_rules! sender_friend {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::FriendSender::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($user_id:expr) => {{
		$crate::FriendSender::builder().user_id($user_id).build()
	}};
}
