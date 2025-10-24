# Changelog

## [0.2.0](https://github.com/puniyu/puniyu/compare/plugin-v0.1.0...plugin-v0.2.0) (2025-10-24)


### ✨ 新功能

* **adapter:** 实现QQ协议适配器API接口 ([#34](https://github.com/puniyu/puniyu/issues/34)) ([18e4f9e](https://github.com/puniyu/puniyu/commit/18e4f9e7245cf8f1355d4f23eca0d2df42e8f7e5))
* **core:** 实现事件驱动和账户信息功能 ([8e05ebd](https://github.com/puniyu/puniyu/commit/8e05ebde81c661f94b4c7599e72971de06c54173))
* **puniyu_macro:** 添加 proc-macro 用于创建插件 ([d933976](https://github.com/puniyu/puniyu/commit/d9339769ae5e91d57a7f0ca0e0d21c1a55c83ad1))
* **task:** 添加定时任务模块 ([e9d823b](https://github.com/puniyu/puniyu/commit/e9d823b5d6ccd089a26166c59e2b67ff8dbd75c1))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息模块 ([5634d99](https://github.com/puniyu/puniyu/commit/5634d99aa2b55841c86f135ba925cf19bc237efd))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))
* **event:** 引入群消息支持并重构消息处理逻辑 ([596848b](https://github.com/puniyu/puniyu/commit/596848b0fa632ad1530ccb645ed6ce14cdf6763f))
