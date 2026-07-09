use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

pub trait Config: Send + Sync + Serialize + for<'de> Deserialize<'de> {
	type Value: Serialize + for<'de> Deserialize<'de>;
	fn name() -> SmolStr;
	fn path() -> PathBuf;
	fn value(&self) -> Self::Value;
}