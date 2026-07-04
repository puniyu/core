# 变更日志

## [0.9.3](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.9.2...puniyu_plugin_core-v0.9.3)

### 🚜 重构


- *(core)* 移除未使用的log依赖并优化trait定义 - ([00df4ca](https://github.com/puniyu/core/commit/00df4cae4fad7f0e4e1fbf8798ff158019fc542b))



### ⚙️ 杂项


- *(core)* 导出puniyu_loader模块 - ([548eb55](https://github.com/puniyu/core/commit/548eb5518cd22e2567a32976621cabbb6bc15a74))




## [0.9.2](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.9.1...puniyu_plugin_core-v0.9.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_command



## [0.9.1](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.9.0...puniyu_plugin_core-v0.9.1)

### ⛰️ 新功能


- 引入AdapterHandle、BotHandle和CommandHandle统一资源管理 ([#286](https://github.com/puniyu/core/pull/286)) - ([baaeede](https://github.com/puniyu/core/commit/baaeede88979c150f4c97868226dd020aa10be3d))



### 🐛 Bug 修复


- Fix - ([d96349b](https://github.com/puniyu/core/commit/d96349be0084ed24c54fc4a0260efca00f84dd61))



### 🚜 重构


- *(command)* 添加命令前缀配置功能 - ([4d87ab5](https://github.com/puniyu/core/commit/4d87ab51f26c4902eb31c363ef75df78a196ade4))


- *(common)* 将日志模块从core迁移到common库 - ([f456f63](https://github.com/puniyu/core/commit/f456f631b20e848e37d0380268745720a74e78a1))



### ⚙️ 杂项


- *(adapter)* 添加API调用功能并重构响应结构 - ([0dc7250](https://github.com/puniyu/core/commit/0dc7250b21da1369d603c6bfb1f4c537aa065d9c))




## [0.9.0](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.18...puniyu_plugin_core-v0.9.0)

### 🚜 重构


- *(config)* 重命名 app 配置为 core 并移除日志配置 - ([f4d73e7](https://github.com/puniyu/core/commit/f4d73e7a0cb44b87230a7e59e74b46eeb056294b))



### ⚙️ 杂项


- [**breaking**] Release v0.9 - ([6a715c7](https://github.com/puniyu/core/commit/6a715c78e0807dc300f4ec8de6d8e234183d17c4))




## [0.8.18](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.17...puniyu_plugin_core-v0.8.18)

### 🐛 Bug 修复


- Trait default impl - ([ecd5aab](https://github.com/puniyu/core/commit/ecd5aab6174f8c7b93064dc42f617b45b4deecc0))



### ⚙️ 杂项


- Update Cargo.lock dependencies



## [0.8.17](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.16...puniyu_plugin_core-v0.8.17)

### 🐛 Bug 修复


- App init - ([aa91efe](https://github.com/puniyu/core/commit/aa91efe38f0406bc48589bb92d5e0266c10bdeaa))



### ⚙️ 杂项


- Update Cargo.lock dependencies



## [0.8.16](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.15...puniyu_plugin_core-v0.8.16)

### ⚙️ 杂项


- Updated the following local packages: puniyu_server, puniyu_command



## [0.8.15](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.14...puniyu_plugin_core-v0.8.15)

### ⚙️ 杂项


- Updated the following local packages: puniyu_version, puniyu_server, puniyu_command



## [0.8.14](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.13...puniyu_plugin_core-v0.8.14)

### ⚙️ 杂项


- Updated the following local packages: puniyu_version, puniyu_server, puniyu_command



## [0.8.13](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.12...puniyu_plugin_core-v0.8.13)

### ⚙️ 杂项


- *(adapter_api)* 移除特定协议APItrait并简化适配器实现 - ([e1f55be](https://github.com/puniyu/core/commit/e1f55be59ffbaaa5c6c2153dc3c7fb371af00f1b))


- Update Cargo.lock dependencies



## [0.8.12](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.11...puniyu_plugin_core-v0.8.12)

### ⚙️ 杂项


- Updated the following local packages: puniyu_version, puniyu_server, puniyu_command



## [0.8.11](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.10...puniyu_plugin_core-v0.8.11)

### 🚜 重构


- *(api)* 移除has_permission宏并重构命令解析器 - ([73bf7db](https://github.com/puniyu/core/commit/73bf7db2b148a5ead6c205db4406dca8805f5129))



### ⚙️ 杂项


- Update Cargo.lock dependencies



## [0.8.10](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.9...puniyu_plugin_core-v0.8.10)

### 🐛 Bug 修复


- Ci - ([03acf4a](https://github.com/puniyu/core/commit/03acf4a1277e6488e8ae2add52378f738d66021b))



### 📚 文档


- 完善doc文档 - ([80f5206](https://github.com/puniyu/core/commit/80f5206397be9d4cdf85a8342361e3971a0331c2))



### ⚙️ 杂项


- *(adapter_api)* 添加Console适配器API并重构OneBot适配器 - ([f37ed65](https://github.com/puniyu/core/commit/f37ed6542ec2c9f6da7f686f5ffca10713e6f6d6))




## [0.8.9](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.8...puniyu_plugin_core-v0.8.9)

### 🐛 Bug 修复


- 修复adapter缺少的导入 - ([a75dee8](https://github.com/puniyu/core/commit/a75dee8b85bd495650f3c33a489d8893ee7f0a24))




## [0.8.8](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.7...puniyu_plugin_core-v0.8.8)

### ⚙️ 杂项


- Updated the following local packages: puniyu_version



## [0.8.6](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.5...puniyu_plugin_core-v0.8.6)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))


- *(core)* 添加Core版本校验 - ([a9f10bd](https://github.com/puniyu/core/commit/a9f10bd9fc71906285496cbe4f5080b27706b808))



### 🐛 Bug 修复


- 修复多余的前缀 - ([0591558](https://github.com/puniyu/core/commit/059155833980811adaf8012d8dbd590c8d4cd1e2))



### 🚜 重构


- *(core)* 移除钩子系统并替换为应用生命周期回调 ([#230](https://github.com/puniyu/core/pull/230)) - ([42cecb9](https://github.com/puniyu/core/commit/42cecb98c2250837db1c1b00ad1a44f3b50e1ece))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))




## [0.8.5](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.4...puniyu_plugin_core-v0.8.5)

### 🚜 重构


- *(app)* 移除未使用的依赖并优化路径处理 - ([471e82b](https://github.com/puniyu/core/commit/471e82b13265a5b2817e386c2e1b52e3347ca44a))


- *(config)* 重构配置trait和注册机制 - ([becb441](https://github.com/puniyu/core/commit/becb4418462a0dcf603101364e683c260e48f871))



### ⚙️ 杂项


- Update Cargo.toml dependencies

- 使用 SmolStr 优化字符串存储和性能 - ([2ee2f25](https://github.com/puniyu/core/commit/2ee2f25ebbff6357443a1b77bef89c5039d7ddab))




## [0.8.4](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.3...puniyu_plugin_core-v0.8.4)

### ⚙️ 杂项


- Updated the following local packages: puniyu_hook, puniyu_server, puniyu_command



## [0.8.3](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.2...puniyu_plugin_core-v0.8.3)

### ⚙️ 杂项


- Updated the following local packages: puniyu_hook, puniyu_server, puniyu_command



## [0.8.2](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.1...puniyu_plugin_core-v0.8.2)

### ⛰️ 新功能


- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


## [0.8.1](https://github.com/puniyu/core/compare/puniyu_plugin_core-v0.8.0...puniyu_plugin_core-v0.8.1)

### ⛰️ 新功能


- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


## [0.8.0]

### ⛰️ 新功能


- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

