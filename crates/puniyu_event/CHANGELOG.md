# 变更日志

## [0.8.2](https://github.com/puniyu/core/compare/puniyu_event-v0.8.1...puniyu_event-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_runtime, puniyu_bot



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_event-v0.8.0...puniyu_event-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- *(puniyu_element)* 添加多种消息段类型支持 - ([b71247b](https://github.com/puniyu/core/commit/b71247beb4cac298e2acffe63b92ec146277730e))
- *(puniyu_event)* 添加通知和请求事件类型支持 - ([d7421d5](https://github.com/puniyu/core/commit/d7421d535c6ea247e6539372e34fda5fa26bb55b))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 独立account模块 - ([0f4c175](https://github.com/puniyu/core/commit/0f4c1758ecc817c2048fec38265edbf630252cb0))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 - ([c403f08](https://github.com/puniyu/core/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(error)* 统一错误处理机制 - ([586272d](https://github.com/puniyu/core/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
- *(event)* 移除SubEventType的Copy派生并优化clone实现 - ([f7a07f0](https://github.com/puniyu/core/commit/f7a07f0aa62b9ebea7fc9a268dc8c09a3a6344d9))
- *(puniyu_element)* 简化ImageElement::new方法的泛型参数 - ([eed53ad](https://github.com/puniyu/core/commit/eed53adbb84b0ad5f4043893a21658415ed63b08))
- *(puniyu_element)* 优化ImageElement::new方法的泛型参数设计 - ([7613ad9](https://github.com/puniyu/core/commit/7613ad99ead915af6cb9cf2683e5c421f4f6a8d0))
- *(runtime)* 重构BotRuntime trait接口 - ([bcfb3c5](https://github.com/puniyu/core/commit/bcfb3c551fab897f84c909a284251487e2b0fc42))
- *(workspace)* 重构项目结构和依赖管理 - ([520087e](https://github.com/puniyu/core/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构工作区crates ([#53](https://github.com/puniyu/core/pull/53)) - ([f55ab51](https://github.com/puniyu/core/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 使用jiff替换chrono库并优化时间处理 ([#179](https://github.com/puniyu/core/pull/179)) - ([8d0257a](https://github.com/puniyu/core/commit/8d0257a9663c43dceafa6e35f7e6f382425df727))

### ⚙️ 杂项


- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))

