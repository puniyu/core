# puniyu_element

消息元素类型库，统一定义接收与发送元素模型。

## 特性

- 提供 `receive` 模块：接收侧元素（消息解析后）
- 提供 `send` 模块：发送侧元素（构造消息）
- 提供统一抽象 `Element` 与 `ElementType`
- 支持序列化与反序列化

## 快速开始

```rust
use puniyu_element::receive::TextElement;
use puniyu_element::send::{TextElement, AtElement, ImageElement, Elements};

// 构造发送元素
let text = TextElement::new("Hello");
let at = AtElement::new("123456");
let image = ImageElement::new(b"image_data".into(), "photo.jpg", None);

// 组合为 Elements 枚举
let elem: Elements = Elements::Text(text);
```