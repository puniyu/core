use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement<'m> {
    /// 图片元素
    pub file: Bytes,
    /// 图片文件名
    pub file_name: &'m str,
    /// 图片外显, 默认拿不到则默认为图片文件名
    pub summary: &'m str,
    /// 图片宽度
    pub width: u32,
    /// 图片高度
    pub height: u32,
}

impl<'m> Element for ImageElement<'m> {
    fn r#type(&self) -> ElementType {
        ElementType::Image
    }
}
