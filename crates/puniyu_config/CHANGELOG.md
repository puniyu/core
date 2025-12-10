# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/config-v0.6.0...config-v0.7.0) (2025-12-10)


### ✨ 新功能

* **types:** 引入 derive_builder 以简化 AdapterInfo 构建 ([9c894fd](https://github.com/puniyu/puniyu/commit/9c894fdf06b49f7f5f73141d03f7769dfc807c5e))


### 🐛 错误修复

* **config:** 修正应用初始化日志未正确应用 ([8c5380b](https://github.com/puniyu/puniyu/commit/8c5380b7a2e5c6f57f79b585eb8ece3bbfcee225))


### ♻️ 代码重构

* **adapter:** 重命名控制台适配器模块路径 ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
* **config:** 更新配置监听器实现 ([939f336](https://github.com/puniyu/puniyu/commit/939f33679e808191b362430db50039f69d0b809a))
* **core:** 重构命令匹配器与上下文处理逻辑 ([3aca600](https://github.com/puniyu/puniyu/commit/3aca600a94a09079ecbd8e84cf51376fc1222a99))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/config-v0.5.12...config-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))


### ♻️ 代码重构

* **config:** 添加群组和好友配置模块, 实现全局cd注册表 ([285cf4a](https://github.com/puniyu/puniyu/commit/285cf4ade69848654fe13cbd14208bc597b040e0))
* **config:** 重构配置模块并新增适配器配置支持 ([26874a2](https://github.com/puniyu/puniyu/commit/26874a22ac9114d487ac56767927b7f1b8bbe205))
* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.12 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/config-v0.5.11...config-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/config-v0.5.10...config-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/config-v0.5.9...config-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/config-v0.5.8...config-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/config-v0.5.7...config-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/config-v0.5.6...config-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/config-v0.5.4...config-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6

## [0.5.4](https://github.com/puniyu/puniyu/compare/config-v0.5.1...config-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.1 to 0.5.4

## [0.5.1](https://github.com/puniyu/puniyu/compare/config-v0.5.0...config-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/config-v0.4.1...config-v0.5.0) (2025-11-23)


### ✨ 新功能

* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))


### ♻️ 代码重构

* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/config-v0.4.0...config-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/config-v0.3.0...config-v0.4.0) (2025-11-16)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/puniyu/puniyu/compare/config-v0.1.8...config-v0.2.0) (2025-11-15)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
