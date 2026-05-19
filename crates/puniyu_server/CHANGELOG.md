# 变更日志

## [0.8.8](https://github.com/puniyu/core/compare/puniyu_server-v0.8.7...puniyu_server-v0.8.8)

### 🐛 Bug 修复


- 修复adapter缺少的导入 - ([a75dee8](https://github.com/puniyu/core/commit/a75dee8b85bd495650f3c33a489d8893ee7f0a24))




## [0.8.5](https://github.com/puniyu/core/compare/puniyu_server-v0.8.4...puniyu_server-v0.8.5)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))


- *(core)* 添加Core版本校验 - ([a9f10bd](https://github.com/puniyu/core/commit/a9f10bd9fc71906285496cbe4f5080b27706b808))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))




## [0.8.4](https://github.com/puniyu/core/compare/puniyu_server-v0.8.3...puniyu_server-v0.8.4)

### 🚜 重构


- *(app)* 移除未使用的依赖并优化路径处理 - ([471e82b](https://github.com/puniyu/core/commit/471e82b13265a5b2817e386c2e1b52e3347ca44a))



### ⚙️ 杂项


- 使用 SmolStr 优化字符串存储和性能 - ([2ee2f25](https://github.com/puniyu/core/commit/2ee2f25ebbff6357443a1b77bef89c5039d7ddab))


- Update Cargo.toml dependencies



## [0.8.3](https://github.com/puniyu/core/compare/puniyu_server-v0.8.2...puniyu_server-v0.8.3)

### ⚙️ 杂项


- Updated the following local packages: puniyu_runtime



## [0.8.2](https://github.com/puniyu/core/compare/puniyu_server-v0.8.1...puniyu_server-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_runtime



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_server-v0.8.0...puniyu_server-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/core/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))

### 🚜 重构


- *(core)* 改进应用关闭逻辑并优化控制台适配器消息处理 - ([03c0e04](https://github.com/puniyu/core/commit/03c0e041fd0b102b74d44ed35ce75b102eadcc06))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- *(puniyu_server)* 重构 logo 模块并添加类型检测 - ([ffb460b](https://github.com/puniyu/core/commit/ffb460b8a3c4d5e6e69f8ac1ce64b8b21e8acb12))
- *(runtime)* 重构ServerRuntime实现并改进服务器生命周期管理 - ([e164bf7](https://github.com/puniyu/core/commit/e164bf7ea370ffef27080f388a9ac5e59d415993))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/core/pull/144)) - ([1447162](https://github.com/puniyu/core/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

### ⚙️ 杂项


- *(workspace)* 移除release-please配置文件并优化Cargo.toml结构 - ([db957c3](https://github.com/puniyu/core/commit/db957c3939f38d30da5cc8807aed0e154fe23a52))
- 使用 bon 替换 derive_builder 并更新依赖 - ([aa29264](https://github.com/puniyu/core/commit/aa29264500d5c3917d7396eef3acaec4df6ad722))

## [0.7.7](https://github.com/puniyu/core/compare/puniyu_server-v0.7.6...puniyu_server-v0.7.7) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types, puniyu_registry - ([0000000](https://github.com/puniyu/core/commit/0000000))

## [0.7.6](https://github.com/puniyu/core/compare/puniyu_server-v0.7.5...puniyu_server-v0.7.6) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))



### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.7.5](https://github.com/puniyu/core/compare/puniyu_server-v0.7.4...puniyu_server-v0.7.5) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))



### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/core/compare/puniyu_server-v0.7.3...puniyu_server-v0.7.4) - 2026-01-07


### ⚙️ 杂项


- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/core/commit/7cd84d6398285b00be4792942110421b71122cbe))




### 贡献者

* @shiwuliya
