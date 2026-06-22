# 变更日志

## [0.8.12](https://github.com/puniyu/core/compare/puniyu_config-v0.8.11...puniyu_config-v0.8.12)

### 🚜 重构


- *(common)* 将日志模块从core迁移到common库 - ([f456f63](https://github.com/puniyu/core/commit/f456f631b20e848e37d0380268745720a74e78a1))



### ⚙️ 杂项


- *(adapter)* 添加API调用功能并重构响应结构 - ([0dc7250](https://github.com/puniyu/core/commit/0dc7250b21da1369d603c6bfb1f4c537aa065d9c))




## [0.8.11](https://github.com/puniyu/core/compare/puniyu_config-v0.8.10...puniyu_config-v0.8.11)

### 🚜 重构


- *(config)* 重命名 app 配置为 core 并移除日志配置 - ([f4d73e7](https://github.com/puniyu/core/commit/f4d73e7a0cb44b87230a7e59e74b46eeb056294b))




## [0.8.10](https://github.com/puniyu/core/compare/puniyu_config-v0.8.9...puniyu_config-v0.8.10)

### 🐛 Bug 修复


- App init - ([aa91efe](https://github.com/puniyu/core/commit/aa91efe38f0406bc48589bb92d5e0266c10bdeaa))




## [0.8.9](https://github.com/puniyu/core/compare/puniyu_config-v0.8.8...puniyu_config-v0.8.9)

### 📚 文档


- 完善doc文档 - ([80f5206](https://github.com/puniyu/core/commit/80f5206397be9d4cdf85a8342361e3971a0331c2))



### ⚙️ 杂项


- *(adapter_api)* 添加Console适配器API并重构OneBot适配器 - ([f37ed65](https://github.com/puniyu/core/commit/f37ed6542ec2c9f6da7f686f5ffca10713e6f6d6))




## [0.8.8](https://github.com/puniyu/core/compare/puniyu_config-v0.8.7...puniyu_config-v0.8.8)

### ⚙️ 杂项


- Updated the following local packages: puniyu_common, puniyu_path



## [0.8.3](https://github.com/puniyu/core/compare/puniyu_config-v0.8.2...puniyu_config-v0.8.3)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))



### 🐛 Bug 修复


- 修复多余的前缀 - ([0591558](https://github.com/puniyu/core/commit/059155833980811adaf8012d8dbd590c8d4cd1e2))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))




## [0.8.2](https://github.com/puniyu/core/compare/puniyu_config-v0.8.1...puniyu_config-v0.8.2)

### 🚜 重构


- *(app)* 移除未使用的依赖并优化路径处理 - ([471e82b](https://github.com/puniyu/core/commit/471e82b13265a5b2817e386c2e1b52e3347ca44a))


- *(config)* 重构配置trait和注册机制 - ([becb441](https://github.com/puniyu/core/commit/becb4418462a0dcf603101364e683c260e48f871))



### ⚙️ 杂项


- 使用 SmolStr 优化字符串存储和性能 - ([2ee2f25](https://github.com/puniyu/core/commit/2ee2f25ebbff6357443a1b77bef89c5039d7ddab))


- Update Cargo.toml dependencies



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_config-v0.8.0...puniyu_config-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🚜 重构


- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/core/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/core/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

### ⚙️ 杂项


- *(workspace)* 移除release-please配置文件并优化Cargo.toml结构 - ([db957c3](https://github.com/puniyu/core/commit/db957c3939f38d30da5cc8807aed0e154fe23a52))

## [0.7.5](https://github.com/puniyu/core/compare/puniyu_config-v0.7.4...puniyu_config-v0.7.5) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/core/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/core/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/core/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/core/compare/puniyu_config-v0.7.3...puniyu_config-v0.7.4) - 2026-01-07


### ⚙️ 杂项


- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/core/commit/7cd84d6398285b00be4792942110421b71122cbe))




### 贡献者

* @shiwuliya
