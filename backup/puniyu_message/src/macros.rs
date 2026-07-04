/// 快速构建 `Message`。
///
/// ```rust
/// use puniyu_message::message;
/// use puniyu_element::send::{AtElement, Elements, TextElement};
///
/// let msg = message!(
///     Elements::At(AtElement::new("123456")),
///     Elements::Text(TextElement::new("hello")),
/// );
/// ```
#[macro_export]
macro_rules! message {
    ($($element:expr),* $(,)?) => {
        $crate::Message::from(vec![$($element),*])
    };
}
