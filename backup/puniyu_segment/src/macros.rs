/// 快速构建消息元素。
///
/// 常用写法：
///
/// ```rust
/// use puniyu_segment::segment;
/// use bytes::Bytes;
///
/// let text = segment!(text, "Hello");
/// let at = segment!(at, "123456");
/// let at_all = segment!(at, all);
/// let face = segment!(face, 123u64);
/// let image = segment!(image, Bytes::from("img"), "photo.jpg");
/// ```
#[macro_export]
macro_rules! segment {
    (at, all) => {
        segment!(at, "all")
    };
    (at, $target_id:expr) => {
        $crate::Segment::at($target_id)
    };
    (text, $text:expr) => {
        $crate::Segment::text($text)
    };
    (image, $file:expr, $file_name:expr) => {
        $crate::Segment::image($file, $file_name, None)
    };
    (image, $file:expr, $file_name:expr, $summary:expr) => {
        $crate::Segment::image($file, $file_name, Some(($summary).into()))
    };
    (reply, $message_id:expr) => {
        $crate::Segment::reply($message_id)
    };
    (file, $file:expr, $file_name:expr) => {
        $crate::Segment::file($file, $file_name)
    };
    (record, $file:expr, $file_name:expr) => {
        $crate::Segment::record($file, $file_name)
    };
    (video, $file:expr, $file_name:expr) => {
        $crate::Segment::video($file, $file_name)
    };
    (face, $id:expr) => {
        $crate::Segment::face($id)
    };
    (json, $data:expr) => {
        $crate::Segment::json($data)
    };
    (xml, $data:expr) => {
        $crate::Segment::xml($data)
    };
    ($($t:tt)*) => {
        compile_error!(concat!("Invalid segment! macro pattern: ", stringify!($($t)*)));
    };
}
