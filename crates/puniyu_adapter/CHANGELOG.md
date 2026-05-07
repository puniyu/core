# 变更日志

## [0.8.3](https://github.com/puniyu/core/compare/puniyu_adapter-v0.8.2...puniyu_adapter-v0.8.3)

### 🚜 重构


- *(puniyu_macros)* 重构宏实现以支持新的钩子系统 ([#209](https://github.com/puniyu/core/pull/209)) - ([1cd15d8](https://github.com/puniyu/core/commit/1cd15d84a59099c7a7cd047d5c320785ad463775))




## [0.8.2](https://github.com/puniyu/core/compare/puniyu_adapter-v0.8.1...puniyu_adapter-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_api - ([0000000](https://github.com/puniyu/core/commit/0000000))


## [0.8.1](https://github.com/puniyu/core/compare/puniyu_adapter-v0.8.0...puniyu_adapter-v0.8.1)

### ⛰️ 新功能


- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/core/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_adapter_console)* 添加控制台适配器的事件分发功能 - ([e16f134](https://github.com/puniyu/core/commit/e16f134395c92d79cc754442d6d2bbf884e64036))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))
- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


## [0.8.0]

### ⛰️ 新功能


- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/core/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))
- *(puniyu_element)* 添加多种消息段类型支持 - ([b71247b](https://github.com/puniyu/core/commit/b71247beb4cac298e2acffe63b92ec146277730e))
- *(puniyu_event)* 添加通知和请求事件类型支持 - ([d7421d5](https://github.com/puniyu/core/commit/d7421d535c6ea247e6539372e34fda5fa26bb55b))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/core/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/core/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(adapter)* 独立account模块 - ([0f4c175](https://github.com/puniyu/core/commit/0f4c1758ecc817c2048fec38265edbf630252cb0))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(command)* 重构命令系统API并优化参数处理 - ([014c60c](https://github.com/puniyu/core/commit/014c60c1658ac6a3624cf7e44d05fe31329981eb))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 - ([c403f08](https://github.com/puniyu/core/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 改进应用关闭逻辑并优化控制台适配器消息处理 - ([03c0e04](https://github.com/puniyu/core/commit/03c0e041fd0b102b74d44ed35ce75b102eadcc06))
- *(error)* 统一错误处理机制 - ([586272d](https://github.com/puniyu/core/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
- *(event)* 移除SubEventType的Copy派生并优化clone实现 - ([f7a07f0](https://github.com/puniyu/core/commit/f7a07f0aa62b9ebea7fc9a268dc8c09a3a6344d9))
- *(puniyu_adapter_console)* 添加控制台适配器的事件分发功能 - ([e16f134](https://github.com/puniyu/core/commit/e16f134395c92d79cc754442d6d2bbf884e64036))
- *(puniyu_element)* 简化ImageElement::new方法的泛型参数 - ([eed53ad](https://github.com/puniyu/core/commit/eed53adbb84b0ad5f4043893a21658415ed63b08))
- *(puniyu_element)* 优化ImageElement::new方法的泛型参数设计 - ([7613ad9](https://github.com/puniyu/core/commit/7613ad99ead915af6cb9cf2683e5c421f4f6a8d0))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- *(puniyu_server)* 重构 logo 模块并添加类型检测 - ([ffb460b](https://github.com/puniyu/core/commit/ffb460b8a3c4d5e6e69f8ac1ce64b8b21e8acb12))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/core/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- *(runtime)* 重构BotRuntime trait接口 - ([bcfb3c5](https://github.com/puniyu/core/commit/bcfb3c551fab897f84c909a284251487e2b0fc42))
- *(runtime)* 重构ServerRuntime实现并改进服务器生命周期管理 - ([e164bf7](https://github.com/puniyu/core/commit/e164bf7ea370ffef27080f388a9ac5e59d415993))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 使用jiff替换chrono库并优化时间处理 ([#179](https://github.com/puniyu/core/pull/179)) - ([8d0257a](https://github.com/puniyu/core/commit/8d0257a9663c43dceafa6e35f7e6f382425df727))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))
- 重构工作区crates ([#53](https://github.com/puniyu/core/pull/53)) - ([f55ab51](https://github.com/puniyu/core/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

