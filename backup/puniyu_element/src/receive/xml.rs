use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement<'x> {
    /// Xml数据，未序列化
    pub data: &'x str,
}

impl<'x> Element for XmlElement<'x> {
    fn r#type(&self) -> ElementType {
        ElementType::Xml
    }
}

impl<'t> From<XmlElement<'t>> for String {
    fn from(XmlElement { data }: XmlElement<'t>) -> Self {
        data.to_string()
    }
}
impl<'t> From<XmlElement<'t>> for &'t str {
    fn from(XmlElement { data }: XmlElement<'t>) -> Self {
        data
    }
}

impl<'t> From<&'t str> for XmlElement<'t> {
    fn from(text: &'t str) -> Self {
        Self { data: text }
    }
}