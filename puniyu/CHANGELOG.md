# Changelog

## [0.2.0](https://github.com/puniyu/puniyu/compare/puniyu-v0.1.0...puniyu-v0.2.0) (2025-10-24)


### ✨ 新功能

* **command:** 支持命令参数解析 ([39359bd](https://github.com/puniyu/puniyu/commit/39359bda42636692ac8d3a680e4795a5fbea16e6))
* **core:** 实现事件驱动和账户信息功能 ([8e05ebd](https://github.com/puniyu/puniyu/commit/8e05ebde81c661f94b4c7599e72971de06c54173))
* **core:** 添加应用启动欢迎界面和插件并行加载 ([6b438da](https://github.com/puniyu/puniyu/commit/6b438dacb03abc83a36a2501c23a01260d06e985))
* **core:** 添加版本信息构建脚本和版本结构体 ([8c3efd2](https://github.com/puniyu/puniyu/commit/8c3efd2897b776447b65fe7147d36dce3b9d2e6e))
* **core:** 重构事件处理和插件系统 ([a62a0ed](https://github.com/puniyu/puniyu/commit/a62a0ed3c658c870c49e7354d992ab13cddd928a))
* **core:** 重构配置模块并添加 Bot 注册功能 ([52c8acb](https://github.com/puniyu/puniyu/commit/52c8acb748c79018429d3f6a787238fd1b2bf14a))
* **core:** 重构配置模块并添加热重载功能 ([eee5739](https://github.com/puniyu/puniyu/commit/eee57396599d5c10ab6cd3520dfc07b5e1fa878d))
* **plugin:** 重构任务管理器以支持动态任务注册与移除 ([7ecb688](https://github.com/puniyu/puniyu/commit/7ecb6883b245b699c2bcc4bbaaad5b4af372959b))
* **plugin:** 重构插件体系并添加任务调度功能 ([5217c68](https://github.com/puniyu/puniyu/commit/5217c68a2ed6c29a889c3199c9992de6c7142e60))
* **puniyu_macro:** 添加 proc-macro 用于创建插件 ([d933976](https://github.com/puniyu/puniyu/commit/d9339769ae5e91d57a7f0ca0e0d21c1a55c83ad1))
* **segment:** 添加消息元素 ([4da83d6](https://github.com/puniyu/puniyu/commit/4da83d6d7d98d44a9c295054dbea228b48a464e0))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))
* **task:** 新增任务宏和任务注册功能 ([59d7b26](https://github.com/puniyu/puniyu/commit/59d7b26c0062f17773ba08acaca01c9ba18f2c10))
* **task:** 添加定时任务模块 ([e9d823b](https://github.com/puniyu/puniyu/commit/e9d823b5d6ccd089a26166c59e2b67ff8dbd75c1))
* **utils:** 新增配置文件处理功能 ([c73a38e](https://github.com/puniyu/puniyu/commit/c73a38efe62140edc2170a33ceae083f069c51de))


### 🔧 其他更新

* **redis:** 添加 Redis 数据库支持 ([80bae19](https://github.com/puniyu/puniyu/commit/80bae191af0620171625a3c0daef38b350403e94))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息模块 ([5634d99](https://github.com/puniyu/puniyu/commit/5634d99aa2b55841c86f135ba925cf19bc237efd))
* **core:** 重构配置模块并优化系统信息获取 ([4d56f79](https://github.com/puniyu/puniyu/commit/4d56f7941e213391189af0da8c05c1383285434d))
* **event:** 引入群消息支持并重构消息处理逻辑 ([596848b](https://github.com/puniyu/puniyu/commit/596848b0fa632ad1530ccb645ed6ce14cdf6763f))
* **message:** 重构消息模块 ([950a65e](https://github.com/puniyu/puniyu/commit/950a65eb774fa1fe8a104a6375193b5e2a1e2f59))
* **message:** 重构音乐元素创建逻辑 ([203e2e7](https://github.com/puniyu/puniyu/commit/203e2e7f84ca78840d8eba41cd931a27990da544))
* 重构项目并更改名称 ([f2d05b1](https://github.com/puniyu/puniyu/commit/f2d05b110676d49ff33237edd0a61f1a77a98652))


### 📦️ 构建系统

* **puniyu_plugin_test:** 增加插件编译时环境变量 ([a003154](https://github.com/puniyu/puniyu/commit/a0031549cf22f2783e6c2c18384cb46d3f37fece))
