# puniyu_server

HTTP 服务能力库，基于 Actix Web 提供可直接运行的服务入口。

## 特性

- 提供 `start_server`、`run_server`、`stop_server`、`restart_server`
- 基于 Actix Web 提供统一服务能力
- 支持命令行参数指定监听地址和端口
- 支持 Logo 设置与获取（`set_logo` / `get_logo`）

## 快速开始

```bash
# 默认启动
cargo run -p puniyu_server

# 指定地址和端口
cargo run -p puniyu_server -- --host 127.0.0.1 --port 33700
```

```rust
use puniyu_server::{run_server, stop_server};

// 启动服务
run_server("127.0.0.1", 33700).await;

// 停止服务
stop_server();
```