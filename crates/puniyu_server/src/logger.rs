#![allow(unused_macros, unused_imports)]

macro_rules! server_info {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::info!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::puniyu_common::core_info!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

pub(crate) use server_info;

macro_rules! server_warn {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::warn!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::puniyu_common::core_warn!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

pub(crate) use server_warn;

macro_rules! server_error {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::error!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::puniyu_common::core_error!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}
pub(crate) use server_error;

macro_rules! server_debug {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::debug!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::puniyu_common::core_debug!!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}
pub(crate) use server_debug;