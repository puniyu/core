#![allow(clippy::unwrap_used)]

use cargo_metadata::MetadataCommand;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	let metadata = MetadataCommand::new().no_deps().exec().unwrap();
	let packages = metadata.packages;
	let name = "puniyu_core";
	let package = packages.iter().find(|p| p.name == name).unwrap();
	let version = &package.version;
	println!("cargo:rustc-env=CORE_VERSION={version}");
	println!("cargo:rustc-env=CORE_VERSION_MAJOR={}", version.major);
	println!("cargo:rustc-env=CORE_VERSION_MINOR={}", version.minor);
	println!("cargo:rustc-env=CORE_VERSION_PATCH={}", version.patch);
}
