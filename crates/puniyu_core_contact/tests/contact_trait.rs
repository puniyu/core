use puniyu_core_contact::Contact;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestScene {
	Friend,
	Group,
	Guild,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestContact {
	scene: TestScene,
	peer: String,
	name: Option<String>,
}

impl Contact for TestContact {
	type Scene = TestScene;

	fn scene(&self) -> Self::Scene {
		self.scene
	}
	fn peer(&self) -> &str {
		&self.peer
	}
	fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}
}

fn friend_alice() -> TestContact {
	TestContact {
		scene: TestScene::Friend,
		peer: "u1".into(),
		name: Some("Alice".into()),
	}
}

fn group_dev() -> TestContact {
	TestContact {
		scene: TestScene::Group,
		peer: "g1".into(),
		name: Some("Dev Team".into()),
	}
}

#[test]
fn trait_method_returns() {
	let c = friend_alice();
	assert_eq!(c.scene(), TestScene::Friend);
	assert_eq!(c.peer(), "u1");
	assert_eq!(c.name(), Some("Alice"));
}

#[test]
fn trait_method_returns_none_name() {
	let c = TestContact {
		scene: TestScene::Guild,
		peer: "gc1".into(),
		name: None,
	};
	assert_eq!(c.name(), None);
	assert_eq!(c.scene(), TestScene::Guild);
}

#[test]
fn blanket_impl_for_ref() {
	let c = group_dev();
	let r: &TestContact = &c;
	let r2: &&TestContact = &r;
	assert_eq!(r.scene(), TestScene::Group);
	assert_eq!(r.peer(), "g1");
	assert_eq!(r2.scene(), TestScene::Group);
	assert_eq!(r2.peer(), "g1");
}

#[test]
fn concrete_partial_eq() {
	assert_eq!(friend_alice(), friend_alice());
	assert_ne!(friend_alice(), group_dev());
}

#[test]
fn scene_is_copy_and_independent() {
	let c = friend_alice();
	let copied: TestScene = c.scene();
	let _still_owned = c;
	assert_eq!(copied, TestScene::Friend);
}