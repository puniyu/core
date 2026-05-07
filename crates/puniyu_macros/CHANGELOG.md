# 变更日志

## [0.8.2](https://github.com/puniyu/core/compare/puniyu_macros-v0.8.1...puniyu_macros-v0.8.2)

### 🚜 重构


- *(puniyu_macros)* 重构宏实现以支持新的钩子系统 ([#209](https://github.com/puniyu/core/pull/209)) - ([1cd15d8](https://github.com/puniyu/core/commit/1cd15d84a59099c7a7cd047d5c320785ad463775))




## [0.8.1](https://github.com/puniyu/core/compare/puniyu_macros-v0.8.0...puniyu_macros-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- *(command)* 添加生命周期参数支持到Arg结构体 - ([c0f28a5](https://github.com/puniyu/core/commit/c0f28a599c7b7e544704dd74cf74dc0194e105f1))
- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/core/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/core/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(command)* 添加命令返回类型验证并优化消息事件处理 - ([8db2173](https://github.com/puniyu/core/commit/8db21733d5fe99034f285bb4035e4dbf30340eee))
- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/core/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))
- 修复文档测试 - ([ed38db1](https://github.com/puniyu/core/commit/ed38db1ff6a8dafb599bd911f5363ac6d3d35bce))

### 🚜 重构


- *(adapter)* 重构适配器注册相关内容 - ([cdec0b9](https://github.com/puniyu/core/commit/cdec0b9e002e12aa6effb6dfda8dc3b331a1fec1))
- *(command)* 重构命令系统API并优化参数处理 - ([014c60c](https://github.com/puniyu/core/commit/014c60c1658ac6a3624cf7e44d05fe31329981eb))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(plugin)* 统一插件初始化返回类型为HandlerResult - ([31c1e58](https://github.com/puniyu/core/commit/31c1e584885121f6464e761d7008ce1548460a57))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/core/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/core/pull/178)) - ([fe74041](https://github.com/puniyu/core/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

### 📚 文档


- *(puniyu_macros)* 更新文档并完善API注释 - ([970a92a](https://github.com/puniyu/core/commit/970a92a87bf99e53c8156c3f7d3a30b3b3c6425a))

### ⚙️ 杂项


- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))

## [0.7.6](https://github.com/puniyu/core/compare/puniyu_macros-v0.7.5...puniyu_macros-v0.7.6) - 2026-01-12


### 🚜 重构


- *(api)* 移除不必要的解引用操作 (由 @wuliya336 提供) - ([7cbd05f](https://github.com/puniyu/core/commit/7cbd05fed5ae6c5a146b0b1cc60528d6a3eb91d9))




### 贡献者

* @wuliya336
## [0.7.5](https://github.com/puniyu/core/compare/puniyu_macros-v0.7.4...puniyu_macros-v0.7.5) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🐛 Bug 修复


- *(macros)* 修复过程宏参数类型 (由 @wuliya336 提供) - ([4091dc4](https://github.com/puniyu/core/commit/4091dc4c57c928a3484fbb72b90626c6338840f3))



### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/core/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/core/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/core/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/core/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/core/compare/puniyu_macros-v0.7.3...puniyu_macros-v0.7.4) - 2026-01-07


### ⚙️ 杂项


- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/core/commit/7cd84d6398285b00be4792942110421b71122cbe))




### 贡献者

* @shiwuliya
