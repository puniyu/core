macro_rules! config_debug {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
        let config_prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::debug!("[{}] [{}] {}", prefix,config_prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_debug;
macro_rules! config_error {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
        let config_prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::error!("[{}] [{}] {}", prefix,config_prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_error;
macro_rules! config_info {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
        let config_prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::info!("[{}] [{}] {}", prefix,config_prefix, format_args!($($arg)+))
	}};
}
pub(crate) use config_info;
