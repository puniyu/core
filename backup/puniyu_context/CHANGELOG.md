# 变更日志

## [0.9.2](https://github.com/puniyu/core/compare/puniyu_context-v0.9.1...puniyu_context-v0.9.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_adapter_core, puniyu_adapter_api, puniyu_adapter_api, puniyu_bot, puniyu_event



## [0.9.1](https://github.com/puniyu/core/compare/puniyu_context-v0.9.0...puniyu_context-v0.9.1)

### ⛰️ 新功能


- 引入AdapterHandle、BotHandle和CommandHandle统一资源管理 ([#286](https://github.com/puniyu/core/pull/286)) - ([baaeede](https://github.com/puniyu/core/commit/baaeede88979c150f4c97868226dd020aa10be3d))



### 🚜 重构


- *(common)* 将日志模块从core迁移到common库 - ([f456f63](https://github.com/puniyu/core/commit/f456f631b20e848e37d0380268745720a74e78a1))



### ⚙️ 杂项


- *(adapter)* 添加API调用功能并重构响应结构 - ([0dc7250](https://github.com/puniyu/core/commit/0dc7250b21da1369d603c6bfb1f4c537aa065d9c))




## [0.9.0](https://github.com/puniyu/core/compare/puniyu_context-v0.8.17...puniyu_context-v0.9.0)

### 🚜 重构


- *(config)* 重命名 app 配置为 core 并移除日志配置 - ([f4d73e7](https://github.com/puniyu/core/commit/f4d73e7a0cb44b87230a7e59e74b46eeb056294b))



### ⚙️ 杂项


- [**breaking**] Release v0.9 - ([6a715c7](https://github.com/puniyu/core/commit/6a715c78e0807dc300f4ec8de6d8e234183d17c4))




## [0.8.17](https://github.com/puniyu/core/compare/puniyu_context-v0.8.16...puniyu_context-v0.8.17)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_adapter_core, puniyu_adapter_api, puniyu_adapter_api, puniyu_runtime, puniyu_runtime, puniyu_bot, puniyu_event



## [0.8.16](https://github.com/puniyu/core/compare/puniyu_context-v0.8.15...puniyu_context-v0.8.16)

### 🐛 Bug 修复


- App init - ([aa91efe](https://github.com/puniyu/core/commit/aa91efe38f0406bc48589bb92d5e0266c10bdeaa))




## [0.8.15](https://github.com/puniyu/core/compare/puniyu_context-v0.8.14...puniyu_context-v0.8.15)

### 🐛 Bug 修复


- Adapter types - ([c4c6805](https://github.com/puniyu/core/commit/c4c6805637483049b757713b5f4f75f00f85ad6d))




## [0.8.14](https://github.com/puniyu/core/compare/puniyu_context-v0.8.13...puniyu_context-v0.8.14)

### ⚙️ 杂项


- *(puniyu_adapter_types)* 使用SmolStr优化内存性能并更新AdapterInfo结构 - ([c4e110b](https://github.com/puniyu/core/commit/c4e110bf3bf0f0ed136858df34fc9513824b1762))




## [0.8.13](https://github.com/puniyu/core/compare/puniyu_context-v0.8.12...puniyu_context-v0.8.13)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_adapter_core, puniyu_adapter_api, puniyu_adapter_api, puniyu_runtime, puniyu_runtime, puniyu_bot, puniyu_event



## [0.8.12](https://github.com/puniyu/core/compare/puniyu_context-v0.8.11...puniyu_context-v0.8.12)

### ⚙️ 杂项


- *(adapter_api)* 移除特定协议APItrait并简化适配器实现 - ([e1f55be](https://github.com/puniyu/core/commit/e1f55be59ffbaaa5c6c2153dc3c7fb371af00f1b))




## [0.8.11](https://github.com/puniyu/core/compare/puniyu_context-v0.8.10...puniyu_context-v0.8.11)

### 🚜 重构


- *(runtime)* 重构运行时模块结构并修复依赖配置 - ([1fa0d3f](https://github.com/puniyu/core/commit/1fa0d3f4456d8be5b97c32e1437b9a7bd2bf686a))




## [0.8.10](https://github.com/puniyu/core/compare/puniyu_context-v0.8.9...puniyu_context-v0.8.10)

### ⚙️ 杂项


- Update Cargo.toml dependencies



## [0.8.9](https://github.com/puniyu/core/compare/puniyu_context-v0.8.8...puniyu_context-v0.8.9)

### 🐛 Bug 修复


- Ci - ([03acf4a](https://github.com/puniyu/core/commit/03acf4a1277e6488e8ae2add52378f738d66021b))


- Fix - ([4748f90](https://github.com/puniyu/core/commit/4748f90767476f3816e73ce4a117867f29323a32))



### 🚜 重构


- 重构适配器API模块 ([#245](https://github.com/puniyu/core/pull/245)) - ([4743d6d](https://github.com/puniyu/core/commit/4743d6d98a22000f84b80c974fde9c821982600a))



### 📚 文档


- 完善doc文档 - ([80f5206](https://github.com/puniyu/core/commit/80f5206397be9d4cdf85a8342361e3971a0331c2))



### ⚙️ 杂项


- *(adapter_api)* 添加Console适配器API并重构OneBot适配器 - ([f37ed65](https://github.com/puniyu/core/commit/f37ed6542ec2c9f6da7f686f5ffca10713e6f6d6))




## [0.8.8](https://github.com/puniyu/core/compare/puniyu_context-v0.8.7...puniyu_context-v0.8.8)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_config, puniyu_runtime, puniyu_bot, puniyu_event



## [0.8.5](https://github.com/puniyu/core/compare/puniyu_context-v0.8.4...puniyu_context-v0.8.5)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))



### 🐛 Bug 修复


- 修复多余的前缀 - ([0591558](https://github.com/puniyu/core/commit/059155833980811adaf8012d8dbd590c8d4cd1e2))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))



### ⚙️ 杂项


- *(message)* 更新消息时间字段类型 - ([1404d4a](https://github.com/puniyu/core/commit/1404d4a9d0887f3dea9546411d4f5363e21dc812))




## [0.8.4](https://github.com/puniyu/core/compare/puniyu_context-v0.8.3...puniyu_context-v0.8.4)

### 🚜 重构


- *(app)* 移除未使用的依赖并优化路径处理 - ([471e82b](https://github.com/puniyu/core/commit/471e82b13265a5b2817e386c2e1b52e3347ca44a))


- *(config)* 重构配置trait和注册机制 - ([becb441](https://github.com/puniyu/core/commit/becb4418462a0dcf603101364e683c260e48f871))



### ⚙️ 杂项


- 使用 SmolStr 优化字符串存储和性能 - ([2ee2f25](https://github.com/puniyu/core/commit/2ee2f25ebbff6357443a1b77bef89c5039d7ddab))


- Update Cargo.toml dependencies



## [0.8.3](https://github.com/puniyu/core/compare/puniyu_context-v0.8.2...puniyu_context-v0.8.3)

### ⛰️ 新功能


- *(app)* 添加应用版本管理功能 - ([6345cdf](https://github.com/puniyu/core/commit/6345cdfbd63898f3ff611f882ed5449bc043e48b))




## [0.8.2](https://github.com/puniyu/core/compare/puniyu_context-v0.8.1...puniyu_context-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_runtime, puniyu_bot, puniyu_event



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_context-v0.8.0...puniyu_context-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- *(puniyu_event)* 添加通知和请求事件类型支持 - ([d7421d5](https://github.com/puniyu/core/commit/d7421d535c6ea247e6539372e34fda5fa26bb55b))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(event)* 移除EventContext中的冗余字段 - ([88ffe3b](https://github.com/puniyu/core/commit/88ffe3b4e39681690553a6c18eb7d0337334435b))
- *(event)* 移除SubEventType的Copy派生并优化clone实现 - ([f7a07f0](https://github.com/puniyu/core/commit/f7a07f0aa62b9ebea7fc9a268dc8c09a3a6344d9))
- *(runtime)* 重构BotRuntime trait接口 - ([bcfb3c5](https://github.com/puniyu/core/commit/bcfb3c551fab897f84c909a284251487e2b0fc42))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

