# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/registry-v0.6.0...registry-v0.7.0) (2025-12-10)


### ✨ 新功能

* **command:** 添加命令权限控制功能 ([cc0013a](https://github.com/puniyu/puniyu/commit/cc0013aff04d8efea0b9cdda3f11eae4d1eac97b))
* **plugin:** 新增服务端插件支持 ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))


### ♻️ 代码重构

* **adapter:** 重构适配器信息结构与初始化逻辑 ([2e45256](https://github.com/puniyu/puniyu/commit/2e4525633031ec401f058507218cb2731ac24479))
* **bus:** 重构事件总线模块 ([7f53e97](https://github.com/puniyu/puniyu/commit/7f53e9731a5f58831c758c372ef46171a8e5208b))
* **config:** 更新配置监听器实现 ([939f336](https://github.com/puniyu/puniyu/commit/939f33679e808191b362430db50039f69d0b809a))
* **registry:** 重构注册表存储结构 ([2627428](https://github.com/puniyu/puniyu/commit/2627428175d8eb79830ec3c98cd8cc7120bc0909))
* **server:** 将服务器命令通道从 tokio mpsc 迁移至 flume ([1345cae](https://github.com/puniyu/puniyu/commit/1345caefb4b07f094d00521fd9286eb6b33b9ab5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.6.0 to 0.7.0
    * puniyu_types bumped from 0.6.0 to 0.7.0
    * puniyu_library bumped from 0.5.1 to 0.5.2
    * puniyu_config bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/registry-v0.5.12...registry-v0.6.0) (2025-12-02)


### ✨ 新功能

* **cli:** 引入命令行参数解析功能 ([11e3137](https://github.com/puniyu/puniyu/commit/11e31372aca53c35f15e8cab8b3067af353d25a7))
* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **macro:** 支持对象格式的命令参数定义 ([672dddd](https://github.com/puniyu/puniyu/commit/672ddddd276eb24f0572bd5a748d01c9a4ae64af))
* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
* **plugin:** 支持命令参数的位置和命名模式 ([85e92d4](https://github.com/puniyu/puniyu/commit/85e92d4ec50367ad3d1e1194ee1542ce74dd82dd))
* **server:** 实现服务器控制功能并完善插件卸载功能 ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))
* **types:** 更新图片元素结构与消息处理逻辑 ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))


### 🐛 错误修复

* **command:** 优化命令行参数错误提示信息 ([3f66a17](https://github.com/puniyu/puniyu/commit/3f66a17c3ec4fa43641f7fe92d31e794b088926a))
* **puniyu_plugin:** 修正plugin宏获取crate name错误 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **puniyu_plugin:** 添加缺少的导入 ([23f7f8a](https://github.com/puniyu/puniyu/commit/23f7f8a459f941971a203063d6215c9779b74411))


### 🎨 代码样式

* **mes:** 优化消息发送日志记录格式 ([0a8336a](https://github.com/puniyu/puniyu/commit/0a8336a777a568c13d27c7e84a5952c40c0d3055))


### ♻️ 代码重构

* **adapter:** 重构适配器存储接口命名 ([eed3b36](https://github.com/puniyu/puniyu/commit/eed3b36ee906f03ac31aec35ec3f5e6d3a038a30))
* **command:** 修正日志格式 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **config:** 添加群组和好友配置模块, 实现全局cd注册表 ([285cf4a](https://github.com/puniyu/puniyu/commit/285cf4ade69848654fe13cbd14208bc597b040e0))
* **config:** 重构配置模块并新增适配器配置支持 ([26874a2](https://github.com/puniyu/puniyu/commit/26874a22ac9114d487ac56767927b7f1b8bbe205))
* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
* **registry:** 重构命令处理器以支持更灵活的消息事件处理 ([1f06ac9](https://github.com/puniyu/puniyu/commit/1f06ac9810a6947533e7218b70590e8c83117add))


### 🎡 持续集成

* **workflows:** 支持可配置的构建模式 ([eed3b36](https://github.com/puniyu/puniyu/commit/eed3b36ee906f03ac31aec35ec3f5e6d3a038a30))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.12 to 0.6.0
    * puniyu_types bumped from 0.5.12 to 0.6.0
    * puniyu_library bumped from 0.5.0 to 0.5.1
    * puniyu_config bumped from 0.5.12 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/registry-v0.5.11...registry-v0.5.12) (2025-11-24)


### 🔧 其他更新

* 修正依赖 ([f16e909](https://github.com/puniyu/puniyu/commit/f16e9098631337f9e47133b5194001ef147c2825))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.11 to 0.5.12
    * puniyu_types bumped from 0.5.11 to 0.5.12
    * puniyu_config bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/registry-v0.5.10...registry-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.10 to 0.5.11
    * puniyu_types bumped from 0.5.10 to 0.5.11
    * puniyu_config bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/registry-v0.5.9...registry-v0.5.10) (2025-11-23)


### ♻️ 代码重构

* **command:** 重构命令处理结果类型和参数验证 ([58d4eeb](https://github.com/puniyu/puniyu/commit/58d4eebb41cacabc7663b40a93181b789feb1e0a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.9 to 0.5.10
    * puniyu_types bumped from 0.5.9 to 0.5.10
    * puniyu_config bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/registry-v0.5.8...registry-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **registry:** 在加载日志中显示插件和适配器版本号 ([0184a74](https://github.com/puniyu/puniyu/commit/0184a74b1dc267690d15bd8e1f6ba25669f7c194))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.8 to 0.5.9
    * puniyu_types bumped from 0.5.8 to 0.5.9
    * puniyu_config bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/registry-v0.5.7...registry-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.7 to 0.5.8
    * puniyu_types bumped from 0.5.7 to 0.5.8
    * puniyu_config bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/registry-v0.5.6...registry-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.6 to 0.5.7
    * puniyu_types bumped from 0.5.6 to 0.5.7
    * puniyu_config bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/registry-v0.5.5...registry-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **deps:** 更新依赖配置 ([84a5c1e](https://github.com/puniyu/puniyu/commit/84a5c1ea0f1d31510a3ee438d4fe408f28686022))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6
    * puniyu_types bumped from 0.5.4 to 0.5.6
    * puniyu_config bumped from 0.5.4 to 0.5.6

## [0.5.5](https://github.com/puniyu/puniyu/compare/registry-v0.5.4...registry-v0.5.5) (2025-11-23)


### 🔧 其他更新

* **puniyu_registry:** 修补依赖 ([e99a202](https://github.com/puniyu/puniyu/commit/e99a202288428974ea55293c189a70bf3f194110))

## [0.5.4](https://github.com/puniyu/puniyu/compare/registry-v0.5.3...registry-v0.5.4) (2025-11-23)


### 🐛 错误修复

* **puniyu_registry:** 补充缺少的依赖 ([8a33e67](https://github.com/puniyu/puniyu/commit/8a33e6775884cf8e65cf3f3baab536d0e3fe4dd6))

## [0.5.3](https://github.com/puniyu/puniyu/compare/registry-v0.5.2...registry-v0.5.3) (2025-11-23)


### 🔧 其他更新

* **core:** 支持设置app_logo ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.1 to 0.5.4
    * puniyu_types bumped from 0.5.1 to 0.5.4
    * puniyu_config bumped from 0.5.1 to 0.5.4

## [0.5.2](https://github.com/puniyu/puniyu/compare/registry-v0.5.1...registry-v0.5.2) (2025-11-23)


### 🔧 其他更新

* **deps:** 添加 futures 依赖项 ([45fc941](https://github.com/puniyu/puniyu/commit/45fc941dced932cbd5e8642f45af1d115d80fea1))

## [0.5.1](https://github.com/puniyu/puniyu/compare/registry-v0.5.0...registry-v0.5.1) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.0 to 0.5.1
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_config bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/registry-v0.4.1...registry-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
* **plugin:** 添加插件配置支持 ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))


### 🐛 错误修复

* **puniyu_registry:** 为 adapter 特性添加 server 支持 ([1c71c3e](https://github.com/puniyu/puniyu/commit/1c71c3e3f223048a55d5bf8bd27f6654e0015064))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_library bumped from 0.4.1 to 0.5.0
    * puniyu_config bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/registry-v0.4.0...registry-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.4.0 to 0.4.1
    * puniyu_bot bumped from 0.4.0 to 0.4.1
    * puniyu_command bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1
    * puniyu_builder bumped from 0.4.0 to 0.4.1
    * puniyu_library bumped from 0.4.0 to 0.4.1
    * puniyu_config bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/registry-v0.3.0...registry-v0.4.0) (2025-11-16)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.1.10 to 0.4.0
    * puniyu_bot bumped from 0.3.0 to 0.4.0
    * puniyu_command bumped from 0.3.0 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0
    * puniyu_builder bumped from 0.3.0 to 0.4.0
    * puniyu_library bumped from 0.3.0 to 0.4.0
    * puniyu_config bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.1.9 to 0.1.10
    * puniyu_bot bumped from 0.2.0 to 0.3.0
    * puniyu_command bumped from 0.2.0 to 0.3.0
    * puniyu_common bumped from 0.2.0 to 0.3.0
    * puniyu_builder bumped from 0.2.0 to 0.3.0
    * puniyu_library bumped from 0.2.0 to 0.3.0
    * puniyu_config bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/puniyu/puniyu/compare/registry-v0.1.8...registry-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))
