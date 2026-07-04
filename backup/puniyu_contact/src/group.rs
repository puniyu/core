//! 群聊联系人模块
//!
//! 提供群聊联系人与群临时联系人的类型定义和构建宏。

mod temp;
pub use temp::*;

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Contact, SceneType};

/// 群聊联系人
///
/// 表示一个群聊的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, GroupContact};
///
/// let group = GroupContact::new("789012", Some("Dev Team"));
/// assert_eq!(group.peer(), "789012");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
pub struct GroupContact<'c> {
	/// 群聊id
	#[builder(into)]
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 群名称
	#[builder(into)]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
}

impl<'c> GroupContact<'c> {
	pub fn new<P, N>(peer: P, name: Option<N>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: name.map(Into::into) }
	}
}

impl<'c> Contact for GroupContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Group
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

/// 构建群聊联系人宏
///
/// 提供便捷的方式创建群聊联系人。
///
/// # 用法
///
/// ## 仅指定 peer
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!("789012");
/// ```
///
/// ## 指定 peer 和 name
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!("789012", "Dev Team");
/// ```
///
/// ## 使用命名字段
///
/// ```rust
/// use puniyu_contact::contact_group;
///
/// let group = contact_group!(
///     peer: "789012",
///     name: "Dev Team",
/// );
/// ```
#[macro_export]
macro_rules! contact_group {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GroupContact::builder().peer($peer).build()
    }};
}
