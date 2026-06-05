use std::borrow::Cow;

/// 适配器标识符。
pub enum AdapterId<'a> {
	/// 通过索引标识。
	Index(u64),
	/// 通过名称标识。
	Name(Cow<'a, str>),
}

impl From<u64> for AdapterId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'a> From<&'a str> for AdapterId<'a> {
	fn from(name: &'a str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for AdapterId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::borrow::Cow;

	#[test]
	fn adapter_id_from_u64_creates_index_variant() {
		let id: AdapterId = 42_u64.into();
		assert!(matches!(id, AdapterId::Index(42)));
	}

	#[test]
	fn adapter_id_from_str_creates_name_variant() {
		let id: AdapterId = "test_adapter".into();
		assert!(matches!(id, AdapterId::Name(Cow::Borrowed("test_adapter"))));
	}

	#[test]
	fn adapter_id_from_string_creates_owned_name_variant() {
		let id: AdapterId = String::from("owned_name").into();
		match id {
			AdapterId::Name(Cow::Owned(ref s)) => assert_eq!(s, "owned_name"),
			_ => panic!("Expected AdapterId::Name(Cow::Owned)"),
		}
	}

	#[test]
	fn adapter_id_from_zero_index() {
		let id: AdapterId = 0_u64.into();
		assert!(matches!(id, AdapterId::Index(0)));
	}

	#[test]
	fn adapter_id_from_empty_str() {
		let id: AdapterId = "".into();
		assert!(matches!(id, AdapterId::Name(Cow::Borrowed(""))));
	}
}
