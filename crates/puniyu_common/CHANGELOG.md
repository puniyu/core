# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/common-v0.6.0...common-v0.7.0) (2025-12-10)


### ✨ 新功能

* **core:** 引入系统信息模块并优化运行时间获取逻辑 ([f0dd848](https://github.com/puniyu/puniyu/commit/f0dd8488d25708e9d01485d28193f1b17cb94dba))


### ♻️ 代码重构

* **puniyu_common:** 添加 stable 特性并重构版本信息模块 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_core:** 移除重复的 version 模块引用 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_server:** 优化 API 路由结构与响应类型泛型化 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))

## [0.6.0](https://github.com/puniyu/puniyu/compare/common-v0.5.12...common-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))


### 🐛 错误修复

* **puniyu_plugin:** 添加缺少的导入 ([23f7f8a](https://github.com/puniyu/puniyu/commit/23f7f8a459f941971a203063d6215c9779b74411))


### ♻️ 代码重构

* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))

## [0.5.12](https://github.com/puniyu/puniyu/compare/common-v0.5.11...common-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.11](https://github.com/puniyu/puniyu/compare/common-v0.5.10...common-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.10](https://github.com/puniyu/puniyu/compare/common-v0.5.9...common-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.9](https://github.com/puniyu/puniyu/compare/common-v0.5.8...common-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.8](https://github.com/puniyu/puniyu/compare/common-v0.5.7...common-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.7](https://github.com/puniyu/puniyu/compare/common-v0.5.6...common-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.6](https://github.com/puniyu/puniyu/compare/common-v0.5.4...common-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.4](https://github.com/puniyu/puniyu/compare/common-v0.5.1...common-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.1](https://github.com/puniyu/puniyu/compare/common-v0.5.0...common-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **common:** Synchronize puniyu versions

## [0.5.0](https://github.com/puniyu/puniyu/compare/common-v0.4.1...common-v0.5.0) (2025-11-23)


### ✨ 新功能

* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **plugin:** 添加插件配置支持 ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))


### 🔧 其他更新

* **deps:** update actions/checkout action to v5.0.1 ([1c68c9c](https://github.com/puniyu/puniyu/commit/1c68c9c096e16d77d1c632e8f6f4b65191878d33))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **config:** 重构配置文件错误处理和TOML操作函数 ([a18e37b](https://github.com/puniyu/puniyu/commit/a18e37b5be5fcf8b64cd461eb21dffdaa7807aab))
* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

## [0.4.1](https://github.com/puniyu/puniyu/compare/common-v0.4.0...common-v0.4.1) (2025-11-16)


### 🐛 错误修复

* **path:** 重构工作目录设置逻辑 ([df51eac](https://github.com/puniyu/puniyu/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))

## [0.4.0](https://github.com/puniyu/puniyu/compare/common-v0.3.0...common-v0.4.0) (2025-11-16)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))

## [0.2.0](https://github.com/puniyu/puniyu/compare/common-v0.1.8...common-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
