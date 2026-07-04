# puniyu_semver

轻量版本号类型库，提供三段式语义版本表示与转换能力。

## 特性

- 提供 `Version` 类型，支持三段式语义版本（Major.Minor.Patch）
- 支持从字符串解析（`Version::parse`）
- 支持比较运算（`Eq`、`Ord`）
- 适合作为统一版本号表示

## 快速开始

```rust
use puniyu_semver::Version;

let v = Version::new(1, 0, 0);
assert_eq!(v.major(), 1);

let parsed = Version::parse("2.3.4").unwrap();
assert!(v < parsed);
```