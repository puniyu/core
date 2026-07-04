pub(crate) trait MergeWith: Sized {
	fn merge_with(&self, global: &Self) -> Self;
}
