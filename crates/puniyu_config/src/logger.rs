macro_rules! config_debug {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::puniyu_common::core_debug!("[{}] {}",prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_debug;
macro_rules! config_error {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::puniyu_common::core_error!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_error;
macro_rules! config_info {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::puniyu_common::core_info!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_info;
