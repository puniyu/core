#[macro_export]
macro_rules! core_trace {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::trace!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

#[macro_export]
macro_rules! core_debug {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::debug!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

#[macro_export]
macro_rules! core_info {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::info!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

#[macro_export]
macro_rules! core_warn {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::warn!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

#[macro_export]
macro_rules! core_error {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Core".fg_rgb::<64, 224, 208>();
		::log::error!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}