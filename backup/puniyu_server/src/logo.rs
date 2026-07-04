use std::sync::{LazyLock, RwLock};

use bytes::Bytes;

pub(crate) static LOGO: LazyLock<RwLock<Bytes>> = LazyLock::new(|| RwLock::new(Bytes::new()));

pub fn set_logo(data: Bytes) {
	if let Ok(mut logo) = LOGO.write() {
		*logo = data;
	}
}

pub fn get_logo() -> Bytes {
	LOGO.read().map(|s| s.clone()).unwrap_or_default()
}
