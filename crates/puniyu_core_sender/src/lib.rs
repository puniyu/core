pub trait Sender: Send + Sync {
	type Sex: Copy + PartialEq + Eq;

	fn user_id(&self) -> &str;
	fn name(&self) -> Option<&str>;
	fn sex(&self) -> Self::Sex;
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

impl<T: Copy + PartialEq + Eq> PartialEq for dyn Sender<Sex = T> {
	fn eq(&self, other: &Self) -> bool {
		self.user_id() == other.user_id()
			&& self.name() == other.name()
			&& self.sex() == other.sex()
			&& self.age() == other.age()
	}
}

impl<T: Copy + PartialEq + Eq> Eq for dyn Sender<Sex = T> {}

