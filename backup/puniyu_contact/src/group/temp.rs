//! 群临时联系人模块
//!
//! 提供群临时联系人的类型定义和构建宏。

use bon::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Contact, SceneType};

/// 群临时联系人
///
/// 表示一个群临时会话的联系信息。
///
/// # 示例
///
/// ```rust
/// use puniyu_contact::{Contact, GroupTempContact};
///
/// let group_temp = GroupTempContact::new("789012", Some("Temp Team"));
/// assert_eq!(group_temp.peer(), "789012");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
pub struct GroupTempContact<'c> {
	/// 群 ID
	#[builder(into)]
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 群名称
	#[builder(into)]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
}

impl<'c> GroupTempContact<'c> {
	pub fn new<P, N>(peer: P, name: Option<N>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: name.map(|s| s.into()) }
	}
}

impl<'c> Contact for GroupTempContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::GroupTemp
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

#[macro_export]
macro_rules! contact_group_temp {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GroupTempContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};

    ($peer:expr, $name:expr) => {{
        $crate::GroupTempContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GroupTempContact::builder().peer($peer).build()
    }};
}
