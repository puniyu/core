#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceType {
	Plugin(u64),
	Adapter(u64),
}
