#![allow(unused_macros)]

use puniyu_common::app::app_name;
use puniyu_config::app_config;
use puniyu_logger::{LogLevel, LoggerOptions};
use std::{env, str::FromStr};

macro_rules! core_trace {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::trace!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
#[allow(unused_imports)]
pub(crate) use core_trace;


macro_rules! core_debug {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::debug!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub(crate) use core_debug;

macro_rules! core_info {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::info!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub(crate) use core_info;

macro_rules! core_warn {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::warn!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
#[allow(unused_imports)]
pub(crate) use core_warn;

macro_rules! core_error {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::error!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub(crate) use core_error;

/// 初始化日志系统
#[cfg(feature = "log")]
pub fn log_init() {
	use puniyu_path::log_dir;
	let config = app_config();
	let logger = config.logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(logger.level().to_string());
	let log_path = log_dir().to_string_lossy().to_string();
	let log_retention_days = logger.retention_days();
	let is_file_logging = logger.enable_file();
	let options = LoggerOptions::default()
		.with_prefix(app_name())
		.with_level(LogLevel::from_str(log_level.as_str()).unwrap_or(LogLevel::Info))
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	puniyu_logger::init(Some(options));
}
