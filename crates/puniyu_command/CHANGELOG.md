# 变更日志

## [0.8.2](https://github.com/puniyu/core/compare/puniyu_command-v0.8.1...puniyu_command-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_context



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_command-v0.8.0...puniyu_command-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- *(command)* 添加生命周期参数支持到Arg结构体 - ([c0f28a5](https://github.com/puniyu/core/commit/c0f28a599c7b7e544704dd74cf74dc0194e105f1))
- *(puniyu_event)* 添加通知和请求事件类型支持 - ([d7421d5](https://github.com/puniyu/core/commit/d7421d535c6ea247e6539372e34fda5fa26bb55b))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- 修正 command_id 初始化索引 - ([7a9bac2](https://github.com/puniyu/core/commit/7a9bac230fa24b936e3130852bb12a4cdbd1be55))

### 🚜 重构


- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/core/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(command)* 重构命令系统API并优化参数处理 - ([014c60c](https://github.com/puniyu/core/commit/014c60c1658ac6a3624cf7e44d05fe31329981eb))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(event)* 移除EventContext中的冗余字段 - ([88ffe3b](https://github.com/puniyu/core/commit/88ffe3b4e39681690553a6c18eb7d0337334435b))
- *(event)* 移除SubEventType的Copy派生并优化clone实现 - ([f7a07f0](https://github.com/puniyu/core/commit/f7a07f0aa62b9ebea7fc9a268dc8c09a3a6344d9))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/core/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- *(runtime)* 重构BotRuntime trait接口 - ([bcfb3c5](https://github.com/puniyu/core/commit/bcfb3c551fab897f84c909a284251487e2b0fc42))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

