use puniyu_semver::Version;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// 应用元信息
#[derive(Debug)]
pub struct AppInfo {
	name: &'static str,
	version: &'static Version,
	cwd_dir: PathBuf,
}

impl AppInfo {
	pub fn new(name: &'static str, version: &'static Version, cwd_dir: impl Into<PathBuf>) -> Self {
		let cwd_dir = cwd_dir.into();
		Self { name, version, cwd_dir }
	}

	pub const fn name(&self) -> &'static str {
		self.name
	}

	pub const fn version(&self) -> &'static Version {
		self.version
	}

	pub fn cwd_dir(&self) -> &Path {
		&self.cwd_dir
	}
}

static APP_INFO: OnceLock<AppInfo> = OnceLock::new();

pub fn set_app_info(info: AppInfo) {
	let _ = APP_INFO.set(info);
}

pub fn app_info() -> &'static AppInfo {
	APP_INFO.get().expect("AppInfo not initialized")
}

pub fn app_name() -> &'static str {
	app_info().name()
}

pub fn app_version() -> &'static Version {
	app_info().version()
}

pub fn app_cwd_dir() -> &'static Path {
	app_info().cwd_dir()
}
