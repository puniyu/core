//! 好友联系人模块
//!
//! 提供好友联系人的类型定义和构建宏。

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Contact, SceneType};

/// 好友联系人
///
/// 表示一个好友的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, FriendContact};
///
/// let friend = FriendContact::new("123456", Some("Alice"));
/// assert_eq!(friend.peer(), "123456");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
pub struct FriendContact<'c> {
	/// 好友ID
	#[builder(into)]
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 好友名称
	#[builder(into)]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
}

impl<'c> FriendContact<'c> {
	pub fn new<P, N>(peer: P, name: Option<N>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: name.map(Into::into) }
	}
}

impl<'c> Contact for FriendContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Friend
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建好友联系人宏
///
/// 提供便捷的方式创建好友联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!("123456");
/// ```
///
/// ## 指定 peer 和 name
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!("123456", "Alice");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_friend;
///
/// let friend = contact_friend!(
///     peer: "123456",
///     name: "Alice",
/// );
/// ```
#[macro_export]
macro_rules! contact_friend {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::FriendContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::FriendContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::FriendContact::builder().peer($peer).build()
    }};
}
