use serde::{Deserialize, Serialize};

use std::{fmt::Debug, hash::Hash};

pub trait Contact: Send + Sync {
	/// 联系人所属的场景
	type Scene: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>;

	/// 获取场景类型
	///
	/// # 返回值
	///
	/// 返回联系人所属的场景引用,具体类型由 [`Self::Scene`] 决定。
	fn scene(&self) -> Self::Scene;

	/// 获取联系人 ID
	///
	/// # 返回值
	///
	/// 返回联系人的唯一标识符 [`str`]。
	fn peer(&self) -> &str;

	/// 获取联系人名称
	///
	/// # 返回值
	///
	/// 返回联系人的名称 [`Option<&str>`],如果未设置则返回 [`None`]。
	fn name(&self) -> Option<&str>;
}

impl<T: Contact + ?Sized> Contact for &T {
	type Scene = T::Scene;
	fn scene(&self) -> Self::Scene {
		(**self).scene()
	}
	fn peer(&self) -> &str {
		(**self).peer()
	}
	fn name(&self) -> Option<&str> {
		(**self).name()
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> PartialEq
	for dyn Contact<Scene = T>
{
	fn eq(&self, other: &Self) -> bool {
		self.scene() == other.scene() && self.peer() == other.peer() && self.name() == other.name()
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Eq
	for dyn Contact<Scene = T>
{
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Hash
	for dyn Contact<Scene = T>
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.scene().hash(state);
		self.peer().hash(state);
		self.name().hash(state);
	}
}

impl<T: Copy + PartialEq + Eq + Hash + Debug + Serialize + for<'de> Deserialize<'de>> Debug
	for dyn Contact<Scene = T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let scene = self.scene();
		f.debug_struct("Contact")
			.field("scene", &scene)
			.field("peer", &self.peer())
			.field("name", &self.name())
			.finish()
	}
}
