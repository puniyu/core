use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 元素类型枚举。
#[derive(
	Debug, Copy, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ElementType {
	/// 艾特元素
	At,
	/// 回复元素
	Reply,
	/// 文本元素
	Text,
	/// 图片元素
	Image,
	/// 文件元素
	File,
	/// 语音元素
	Record,
	/// 视频元素
	Video,
	/// 表情元素
	Face,
	/// json元素
	Json,
	/// xml元素
	Xml,
}

/// 元素统一接口。
pub trait Element: Send + Sync {
	/// 获取元素类型。
	fn r#type(&self) -> ElementType;
}

impl PartialEq for dyn Element {
	fn eq(&self, other: &Self) -> bool {
		self.r#type() == other.r#type()
	}
}
impl Eq for dyn Element {}
