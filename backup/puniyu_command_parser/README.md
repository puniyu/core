# puniyu_command_parser

命令解析器，负责解析命令文本、别名和参数。

## 特性

- 支持命令文本解析与参数提取
- 支持命令别名剥离
- 支持参数验证与类型转换
- 提供 `CommandParseResult` 解析结果类型

## 快速开始

```rust
use puniyu_command_parser::CommandParser;

let parser = CommandParser::new();
let result = parser.parse("/help arg1 arg2").unwrap();
```