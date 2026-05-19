# 变更日志

## [0.8.9](https://github.com/puniyu/core/compare/v0.8.8...v0.8.9)

### 🐛 Bug 修复


- 修复adapter缺少的导入 - ([a75dee8](https://github.com/puniyu/core/commit/a75dee8b85bd495650f3c33a489d8893ee7f0a24))




## [0.8.8](https://github.com/puniyu/core/compare/v0.8.7...v0.8.8)

### 🐛 Bug 修复


- 修复导入 - ([aca982b](https://github.com/puniyu/core/commit/aca982b1658672d5562103d653dd3956a45437c9))




## [0.8.6](https://github.com/puniyu/core/compare/v0.8.5...v0.8.6)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))


- *(core)* 添加Core版本校验 - ([a9f10bd](https://github.com/puniyu/core/commit/a9f10bd9fc71906285496cbe4f5080b27706b808))



### 🐛 Bug 修复


- 修复多余的前缀 - ([0591558](https://github.com/puniyu/core/commit/059155833980811adaf8012d8dbd590c8d4cd1e2))



### 🚜 重构


- *(core)* 移除钩子系统并替换为应用生命周期回调 ([#230](https://github.com/puniyu/core/pull/230)) - ([42cecb9](https://github.com/puniyu/core/commit/42cecb98c2250837db1c1b00ad1a44f3b50e1ece))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))



### ⚙️ 杂项


- *(message)* 更新消息时间字段类型 - ([1404d4a](https://github.com/puniyu/core/commit/1404d4a9d0887f3dea9546411d4f5363e21dc812))




## [0.8.5](https://github.com/puniyu/core/compare/v0.8.4...v0.8.5)

### 🚜 重构


- *(app)* 移除未使用的依赖并优化路径处理 - ([471e82b](https://github.com/puniyu/core/commit/471e82b13265a5b2817e386c2e1b52e3347ca44a))


- *(config)* 重构配置trait和注册机制 - ([becb441](https://github.com/puniyu/core/commit/becb4418462a0dcf603101364e683c260e48f871))


- *(puniyu_core)* 导出所有公开模块 - ([bd55400](https://github.com/puniyu/core/commit/bd554000ba981327a7c5b89169c1a703e5c3c58a))



### ⚙️ 杂项


- 使用 SmolStr 优化字符串存储和性能 - ([2ee2f25](https://github.com/puniyu/core/commit/2ee2f25ebbff6357443a1b77bef89c5039d7ddab))


- Update Cargo.toml dependencies



## [0.8.4](https://github.com/puniyu/core/compare/v0.8.3...v0.8.4)

### ⛰️ 新功能


- *(app)* 添加应用版本管理功能 - ([6345cdf](https://github.com/puniyu/core/commit/6345cdfbd63898f3ff611f882ed5449bc043e48b))



### 🚜 重构


- *(core)* 重构应用结构并改进日志系统 - ([eb562a8](https://github.com/puniyu/core/commit/eb562a8c02b619b3bda82f5c8fe3bb20f5b3a2f6))




## [0.8.3](https://github.com/puniyu/core/compare/v0.8.2...v0.8.3)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_hook, puniyu_server, puniyu_command, puniyu_plugin_core, puniyu_api, puniyu_handler, puniyu_dispatch, puniyu_loader



## [0.8.2](https://github.com/puniyu/core/compare/v0.8.1...v0.8.2)

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


## [0.8.1](https://github.com/puniyu/core/compare/v0.8.0...v0.8.1)

### ⛰️ 新功能


- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/core/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


## [0.8.0]

### ⛰️ 新功能


- *(command)* 添加生命周期参数支持到Arg结构体 - ([c0f28a5](https://github.com/puniyu/core/commit/c0f28a599c7b7e544704dd74cf74dc0194e105f1))
- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/core/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- *(puniyu_ipc)* 添加IPC通信功能 - ([f17bee5](https://github.com/puniyu/core/commit/f17bee5efd8b472825ac55eb2f0ce0ba69820021))
- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/core/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- *(app)* 修改配置文件初始化逻辑 - ([75ae0da](https://github.com/puniyu/core/commit/75ae0da80ea2c73b45153c488823f13a0f57a388))
- *(core)* 更新应用logo设置逻辑 - ([b805b79](https://github.com/puniyu/core/commit/b805b79c71198d5acb56daa892e6eef825f60b8b))
- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))
- Fix - ([cd76df1](https://github.com/puniyu/core/commit/cd76df1dea0e8219479b18bda687dc03e9acff48))
- 修正 command_id 初始化索引 - ([7a9bac2](https://github.com/puniyu/core/commit/7a9bac230fa24b936e3130852bb12a4cdbd1be55))

### 🚜 重构


- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/core/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(command)* 重构命令系统API并优化参数处理 - ([014c60c](https://github.com/puniyu/core/commit/014c60c1658ac6a3624cf7e44d05fe31329981eb))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 - ([c403f08](https://github.com/puniyu/core/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 改进应用关闭逻辑并优化控制台适配器消息处理 - ([03c0e04](https://github.com/puniyu/core/commit/03c0e041fd0b102b74d44ed35ce75b102eadcc06))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_adapter_console)* 重命名ConsoleAdapterRuntime为Runtime - ([3cc5211](https://github.com/puniyu/core/commit/3cc52116562161b4acf437b0382c30b7ba70bac0))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- *(puniyu_server)* 重构 logo 模块并添加类型检测 - ([ffb460b](https://github.com/puniyu/core/commit/ffb460b8a3c4d5e6e69f8ac1ce64b8b21e8acb12))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/core/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- *(runtime)* 重构ServerRuntime实现并改进服务器生命周期管理 - ([e164bf7](https://github.com/puniyu/core/commit/e164bf7ea370ffef27080f388a9ac5e59d415993))
- *(workspace)* 重构项目结构和依赖管理 - ([520087e](https://github.com/puniyu/core/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 使用jiff替换chrono库并优化时间处理 ([#179](https://github.com/puniyu/core/pull/179)) - ([8d0257a](https://github.com/puniyu/core/commit/8d0257a9663c43dceafa6e35f7e6f382425df727))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))
- 重构工作区crates ([#53](https://github.com/puniyu/core/pull/53)) - ([f55ab51](https://github.com/puniyu/core/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚡ 性能优化


- *(app)* 更新工作目录处理逻辑 - ([5be34fe](https://github.com/puniyu/core/commit/5be34fe7e2df1b4e9de5bff2e9d0c775f4413ef0))

### ⚙️ 杂项


- *(workspace)* 移除release-please配置文件并优化Cargo.toml结构 - ([db957c3](https://github.com/puniyu/core/commit/db957c3939f38d30da5cc8807aed0e154fe23a52))
- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))
- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))

## [0.7.7](https://github.com/puniyu/core/compare/core-v0.7.6...core-v0.7.7) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types, puniyu_registry, puniyu_event, puniyu_server - ([0000000](https://github.com/puniyu/core/commit/0000000))

## [0.7.6](https://github.com/puniyu/core/compare/core-v0.7.5...core-v0.7.6) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/core/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/core/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/core/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.7.5](https://github.com/puniyu/core/compare/core-v0.7.4...core-v0.7.5) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/core/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/core/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/core/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/core/compare/core-v0.7.3...core-v0.7.4) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 支持配置文件读取功能 (由 @shiwuliya 提供) - ([a9fc6e2](https://github.com/puniyu/core/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
- *(cli)* 引入命令行参数解析功能 (由 @shiwuliya 提供) - ([11e3137](https://github.com/puniyu/core/commit/11e31372aca53c35f15e8cab8b3067af353d25a7))
- *(config)* 重构配置管理模块以支持动态注册与热重载 (由 @shiwuliya 提供) - ([f3234c1](https://github.com/puniyu/core/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
- *(core)* 引入系统信息模块并优化运行时间获取逻辑 (由 @shiwuliya 提供) - ([f0dd848](https://github.com/puniyu/core/commit/f0dd8488d25708e9d01485d28193f1b17cb94dba))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 (由 @shiwuliya 提供) - ([f49677a](https://github.com/puniyu/core/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/core/pull/93)) (由 @shiwuliya 提供) (#93) - ([9611fba](https://github.com/puniyu/core/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(protocol)* 实现事件协议和字节数据类型支持 (由 @shiwuliya 提供) (#91) - ([ec854ca](https://github.com/puniyu/core/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
- *(protocol)* 添加联系人和发送者协议支持并重构元素处理 (由 @shiwuliya 提供) (#91) - ([e3e6bba](https://github.com/puniyu/core/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
- *(server)* 实现服务器控制功能并完善插件卸载功能 (由 @shiwuliya 提供) - ([3cab133](https://github.com/puniyu/core/commit/3cab133589d93d2d7592ed867db245999c774723))
- *(types)* 引入 derive_builder 以简化 AdapterInfo 构建 (由 @shiwuliya 提供) - ([9c894fd](https://github.com/puniyu/core/commit/9c894fdf06b49f7f5f73141d03f7769dfc807c5e))



### 🐛 Bug 修复


- *(path)* 重构工作目录设置逻辑 (由 @shiwuliya 提供) - ([df51eac](https://github.com/puniyu/core/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))

- 初始化时间计算错误 (由 @shiwuliya 提供) - ([2c53605](https://github.com/puniyu/core/commit/2c536057bc8c59649863117a0e542649faad3adc))


### 🚜 重构


- *(adapter)* 优化 Avatar 类型实现并添加服务器 logo 接口 (由 @shiwuliya 提供) - ([952c18b](https://github.com/puniyu/core/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
- *(app)* 重构应用构建器和适配器加载逻辑 (由 @shiwuliya 提供) - ([58ddb3f](https://github.com/puniyu/core/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(bus)* 重构事件总线模块 (由 @shiwuliya 提供) - ([7f53e97](https://github.com/puniyu/core/commit/7f53e9731a5f58831c758c372ef46171a8e5208b))
- *(config)* 添加适配器配置系统支持 (由 @shiwuliya 提供) - ([5358888](https://github.com/puniyu/core/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
- *(config)* 重构配置文件监听器实现 (由 @shiwuliya 提供) - ([4c99137](https://github.com/puniyu/core/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
- *(core)* 重构元素宏和消息构建器实现 (由 @shiwuliya 提供) - ([abdd4ac](https://github.com/puniyu/core/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 (由 @shiwuliya 提供) - ([4e2d787](https://github.com/puniyu/core/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(core)* 调整Bot模块导出方式 (由 @shiwuliya 提供) - ([d94d88c](https://github.com/puniyu/core/commit/d94d88cf891d9e222fb996c54895fa062bc17fe7))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/core/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(project)* 重构项目结构 (由 @shiwuliya 提供) - ([b1b389f](https://github.com/puniyu/core/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(puniyu_common)* 添加 stable 特性并重构版本信息模块 (由 @shiwuliya 提供) - ([5509982](https://github.com/puniyu/core/commit/550998233d7ee02150b15c16f7ef3fa139086137))

- 重构工作区crates ([#53](https://github.com/puniyu/core/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/core/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### 🎨 样式


- *(code)* 项目格式化 (由 @shiwuliya 提供) - ([dc3d850](https://github.com/puniyu/core/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))



### ⚙️ 杂项


- *(core)* 支持设置app_logo (由 @shiwuliya 提供) - ([3f4f713](https://github.com/puniyu/core/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(core)* 导出 puniyu_common 的 path 模块 (由 @shiwuliya 提供) - ([a95c43e](https://github.com/puniyu/core/commit/a95c43e1104b4bb4aadaeaddc3d07f43dc083968))
- *(puniyu_core)* 更新 Cargo.toml 配置 (由 @shiwuliya 提供) - ([2c77c7a](https://github.com/puniyu/core/commit/2c77c7a4b6dfc08e23835ae3ba29227c0a8ea43f))
- *(puniyu_logger)* 更新 puniyu_logger 依赖版本 (由 @shiwuliya 提供) - ([f949ad4](https://github.com/puniyu/core/commit/f949ad40d00b11b9caa3f78fecb35fc37055742f))
- *(puniyu_types)* 更新 AdapterInfo 结构体字段默认值及构造宏 (由 @shiwuliya 提供) (#91) - ([76e8ebe](https://github.com/puniyu/core/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))
- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/core/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 (由 @shiwuliya 提供) - ([fd81025](https://github.com/puniyu/core/commit/fd81025eee0ef359320d1a8c482004772491ce0f))



### Refcator



- 优化bot实例的使用 (由 @shiwuliya 提供) (#72) - ([73f284e](https://github.com/puniyu/core/commit/73f284e8c594139d2a190fc09cb7ba460ceb4ef8))



### 贡献者

* @shiwuliya
* @puniyu[bot]
