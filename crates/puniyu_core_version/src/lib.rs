use puniyu_core_semver::Version;

pub const VERSION: Version = Version {
	major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
	minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
	patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
};
