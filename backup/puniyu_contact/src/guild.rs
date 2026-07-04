use bon::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{Contact, SceneType};

/// 频道联系人
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Builder)]
#[serde(bound(deserialize = "'de: 'c"))]
pub struct GuildContact<'c> {
	/// 频道id
	#[builder(into)]
	#[serde(borrow)]
	peer: Cow<'c, str>,
	/// 频道名称
	#[builder(into)]
	#[serde(borrow)]
	name: Option<Cow<'c, str>>,
	/// 子频道名称
	#[builder(into)]
	#[serde(borrow)]
	sub_name: Option<Cow<'c, str>>,
}

impl<'c> GuildContact<'c> {
	pub fn new<P, N, S>(peer: P, name: Option<N>, sub_name: Option<S>) -> Self
	where
		P: Into<Cow<'c, str>>,
		N: Into<Cow<'c, str>>,
		S: Into<Cow<'c, str>>,
	{
		Self { peer: peer.into(), name: name.map(Into::into), sub_name: sub_name.map(Into::into) }
	}

	/// 获取子频道名称
	pub fn sub_name(&self) -> Option<&str> {
		self.sub_name.as_deref()
	}
}

impl<'c> Contact for GuildContact<'c> {
	fn scene(&self) -> &SceneType {
		&SceneType::Guild
	}

	fn peer(&self) -> &str {
		self.peer.as_ref()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

#[macro_export]
macro_rules! contact_guild {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::GuildContact::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
    ($peer:expr, $name:expr, sub_name: $sub_name:expr) => {{
        $crate::GuildContact::builder()
            .peer($peer)
            .name($name)
			.sub_name($sub_name)
            .build()
    }};
    ($peer:expr, $name:expr) => {{
        $crate::GuildContact::builder().peer($peer).name($name).build()
    }};

    ($peer:expr) => {{
        $crate::GuildContact::builder().peer($peer).build()
    }};
}
