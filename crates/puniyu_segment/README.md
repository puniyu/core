# puniyu_segment

消息段构建库，提供 `Segment` 结构体和 `segment!` 宏快速构建 `Elements`。

## 特性

- 提供 `Segment` 结构体方法构建消息段
- 支持 `segment!` 宏快速生成 `Elements`
- 支持文本、At、表情、图片、语音、文件、视频、JSON、XML 等类型
- 返回 `Elements` 类型，与 `puniyu_message` 配合使用

## 快速开始

```rust
use puniyu_segment::{Segment, segment};
use bytes::Bytes;

let at = Segment::at("123456");
let text = segment!(text, "Hello");
let face = segment!(face, 123u64);
let image = segment!(image, Bytes::from("img"), "photo.jpg");
```