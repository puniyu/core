mod sex;
use std::fmt::Debug;

#[doc(inline)]
pub use sex::Sex;
mod role;
#[doc(inline)]
pub use role::Role;

/// 发送者统一接口。
pub trait Sender: Send + Sync {
	/// 获取发送者 ID。
	fn user_id(&self) -> &str;

	/// 获取发送者昵称。
	fn name(&self) -> Option<&str>;

	/// 获取发送者性别。
	fn sex(&self) -> &Sex;

	/// 获取发送者年龄。
	fn age(&self) -> Option<u32>;
}

impl<T: Sender> Sender for &T {
	fn user_id(&self) -> &str {
		(**self).user_id()
	}

	fn name(&self) -> Option<&str> {
		(**self).name()
	}

	fn sex(&self) -> &Sex {
		(**self).sex()
	}

	fn age(&self) -> Option<u32> {
		(**self).age()
	}
}

impl PartialEq for dyn Sender {
	fn eq(&self, other: &Self) -> bool {
		self.user_id() == other.user_id()
			&& self.name() == other.name()
			&& self.sex() == other.sex()
			&& self.age() == other.age()
	}
}

impl Debug for dyn Sender {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Sender")
			.field("user_id", &self.user_id())
			.field("name", &self.name())
			.field("sex", &self.sex())
			.field("age", &self.age())
			.finish()
	}
}
