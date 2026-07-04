use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum PluginId<'p> {
	Index(u64),
	Name(Cow<'p, str>),
}

impl From<u64> for PluginId<'_> {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'p> From<&'p str> for PluginId<'p> {
	fn from(value: &'p str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for PluginId<'_> {
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}
