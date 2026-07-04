use crate::{EventBase, SubEventType};
use derive_more::{Debug, Display, From};
use std::borrow::Cow;

#[derive(Clone, PartialEq, Eq, From, Display, Debug)]
#[display("{}", _0)]
#[debug("{}", _0)]
pub struct NoticeSubEventType(Cow<'static, str>);

impl NoticeSubEventType {
	pub fn new(kind: impl Into<Cow<'static, str>>) -> Self {
		Self(kind.into())
	}

	pub fn kind(&self) -> &str {
		self.0.as_ref()
	}
}

#[derive(Clone, PartialEq, Eq, From, Display, Debug)]
#[display("{}", _0)]
#[debug("{}", _0)]
pub struct RequestSubEventType(Cow<'static, str>);

impl RequestSubEventType {
	pub fn new(kind: impl Into<Cow<'static, str>>) -> Self {
		Self(kind.into())
	}

	pub fn kind(&self) -> &str {
		self.0.as_ref()
	}
}

pub trait ExtensionEvent: EventBase + Send + Sync + 'static {
	fn r#type(&self) -> SubEventType;

	fn content(&self) -> &str;
}
