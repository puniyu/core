use serde::{Deserialize, Serialize};

use std::{fmt::Debug, hash::Hash};

pub trait Sender: Send + Sync {
	/// 发送者的性别
	type Sex: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>;

	/// 获取发送者 ID。
	fn user_id(&self) -> &str;

	/// 获取发送者昵称。
	fn name(&self) -> Option<&str>;

	/// 获取发送者性别。
	fn sex(&self) -> Self::Sex;

	/// 获取发送者年龄。
	fn age(&self) -> Option<u32>;
}

impl<T: Sender + ?Sized> Sender for &T {
	type Sex = T::Sex;
	fn user_id(&self) -> &str {
		(**self).user_id()
	}
	fn name(&self) -> Option<&str> {
		(**self).name()
	}
	fn sex(&self) -> Self::Sex {
		(**self).sex()
	}
	fn age(&self) -> Option<u32> {
		(**self).age()
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> PartialEq for dyn Sender<Sex = T> {
	fn eq(&self, other: &Self) -> bool {
		self.user_id() == other.user_id()
			&& self.name() == other.name()
			&& self.sex() == other.sex()
			&& self.age() == other.age()
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Eq for dyn Sender<Sex = T> {}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Hash for dyn Sender<Sex = T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.user_id().hash(state);
		self.name().hash(state);
		self.sex().hash(state);
		self.age().hash(state);
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Debug for dyn Sender<Sex = T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let sex = self.sex();
		f.debug_struct("Sender")
			.field("user_id", &self.user_id())
			.field("name", &self.name())
			.field("sex", &sex)
			.field("age", &self.age())
			.finish()
	}
}


