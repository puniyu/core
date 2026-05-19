//! # puniyu_semver
//!
//! 轻量语义版本类型，主要用于在项目内部共享 `major.minor.patch` 版本信息。
//!
//! ## 特性
//!
//! - 提供 `Version::new` 构造函数
//! - 支持 `Display`，格式为 `major.minor.patch`
//! - 支持 `FromStr` 解析语义化版本字符串
//! - 支持与 `semver::Version` 双向转换
//! - 支持 `serde` 序列化与反序列化
//!
//! ## 设计说明
//!
//! `Version` 仅保留语义化版本的核心三段（`major` / `minor` / `patch`）。
//! 当从 `semver::Version` 转换或解析包含预发布/构建元数据的字符串时，
//! 这些扩展信息会被忽略。
//!
//! 例如：`1.2.3-beta.1+build.7` 会被转换为 `Version::new(1, 2, 3)`。
//!
//! ## 示例
//!
//! ```rust
//! use std::str::FromStr;
//! use puniyu_semver::Version;
//!
//! let version = Version::new(1, 2, 3);
//! assert_eq!(version.to_string(), "1.2.3");
//!
//! let parsed = Version::from_str("1.2.3-beta.1+build.7").unwrap();
//! assert_eq!(parsed, Version::new(1, 2, 3));
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// 三段式版本号（`major.minor.patch`）。
///
/// 这是 `semver` 的简化表示，仅保存核心版本信息，不保存预发布和构建元数据。
#[derive(
	Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display,
)]
#[display("{major}.{minor}.{patch}")]
pub struct Version {
	/// 主版本号
	pub major: u64,
	/// 次版本号
	pub minor: u64,
	/// 补丁版本号
	pub patch: u64,
}

impl Version {
	/// 创建一个新的版本号。
	pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
		Self { major, minor, patch }
	}

	pub fn parse(s: &str) -> Result<Self, semver::Error> {
		Ok(Self::from(semver::Version::parse(s)?))
	}
}

impl From<semver::Version> for Version {
	fn from(v: semver::Version) -> Self {
		Self { major: v.major, minor: v.minor, patch: v.patch }
	}
}

impl From<Version> for semver::Version {
	fn from(v: Version) -> Self {
		semver::Version::new(v.major, v.minor, v.patch)
	}
}

impl FromStr for Version {
	type Err = semver::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		semver::Version::from_str(s).map(Into::into)
	}
}

