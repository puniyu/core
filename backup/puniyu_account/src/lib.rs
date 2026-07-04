#![allow(clippy::duplicated_attributes)]

//! # puniyu_account
//!
//! 统一的机器人账户信息类型，描述账号 UIN、昵称与头像数据。
//!
//! ## 特性
//!
//! - 提供 `AccountInfo`
//! - 提供构建宏 `account_info!`
//! - 支持 `serde` 序列化与反序列化
//! - 提供 `AccountInfoBuilder`
//!
//! ## 示例
//!
//! ```rust
//! use bytes::Bytes;
//! use puniyu_account::AccountInfo;
//!
//! let account = AccountInfo {
//!     uin: "123456789".to_string(),
//!     name: "Puniyu".to_string(),
//!     avatar: Bytes::from_static(b"avatar"),
//! };
//! assert_eq!(account.uin, "123456789");
//! ```
//!
//! ```rust
//! use bytes::Bytes;
//! use puniyu_account::account_info;
//!
//! let account = account_info!(
//!     uin: "123456789",
//!     name: "Puniyu",
//!     avatar: Bytes::from_static(b"avatar"),
//! );
//! assert_eq!(account.name, "Puniyu");
//! ```

use bon::Builder;
use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// 机器人账户信息。
///
/// 统一描述当前机器人账号的标识、展示名称和头像数据。
///
/// # 示例
///
/// ```rust
/// use bytes::Bytes;
/// use puniyu_account::AccountInfo;
///
/// let account = AccountInfo {
///     uin: "123456789".to_string(),
///     name: "Puniyu".to_string(),
///     avatar: Bytes::from_static(b"avatar"),
/// };
///
/// assert_eq!(account.uin, "123456789");
/// assert_eq!(account.name, "Puniyu");
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(on(String, into), on(Bytes, into))]
pub struct AccountInfo {
	/// 机器人账号的唯一标识 UIN。
	pub uin: String,
	/// 机器人账号的展示名称。
	pub name: String,
	/// 机器人头像的原始字节数据。
	pub avatar: Bytes,
}

/// 构建机器人账户信息。
///
/// 支持命名字段与位置参数两种写法。
///
/// ```rust
/// use bytes::Bytes;
/// use puniyu_account::account_info;
///
/// let named = account_info!(
///     uin: "123456789",
///     name: "Puniyu",
///     avatar: Bytes::from_static(b"avatar"),
/// );
/// assert_eq!(named.uin, "123456789");
///
/// let positional = account_info!(
///     "123456789",
///     "Puniyu",
///     Bytes::new()
/// );
/// assert_eq!(positional.name, "Puniyu");
/// ```
#[macro_export]
macro_rules! account_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::AccountInfo::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($uin:expr, $name:expr, $avatar:expr) => {{
		$crate::account_info!(
			uin: $uin,
			name: $name,
			avatar: $avatar,
		)
	}};
}
