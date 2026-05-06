![puniyu](packages/puniyu/assets/logo.png)

# puniyu

一个基于 Rust 的模块化机器人框架，围绕事件、适配器、插件和处理器构建

## 特性

- 模块化 workspace 架构，核心能力按 crate 拆分
- 统一的事件、上下文、命令与运行时抽象
- 支持适配器、插件、处理器、Hook、任务等扩展点
- 内置控制台适配器与基础插件，可直接本地运行

## 快速开始

### 环境要求

- Rust `1.88.0` 或更高版本
- Cargo

### 运行示例

```bash
cargo run -p puniyu
```

默认启动入口位于 [packages/puniyu/src/main.rs](packages/puniyu/src/main.rs)


## 社区与链接

- GitHub：<https://github.com/puniyu/core>
- DeepWiki：<https://deepwiki.com/puniyu/puniyu>
- QQ 群：`1022851882`
