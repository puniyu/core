# puniyu_message

消息链封装库，统一组合 `Elements` 构建消息。

## 特性

- 提供 `Message` 类型，封装 `Vec<Elements>`
- 支持 `message!` 宏快速构建消息链
- 支持从 `&str`、`String`、`Vec<Elements>` 等类型转换
- 提供格式化输出

## 快速开始

```rust
use puniyu_message::{message, Message};
use puniyu_element::send::{Elements, TextElement};

let msg = message!(
    Elements::Text(TextElement::new("Hello")),
    Elements::Text(TextElement::new(" puniyu!")),
);

// 也可直接从字符串创建
let msg: Message = "hello".into();
```