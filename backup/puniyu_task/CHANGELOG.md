# 变更日志

## [0.8.10](https://github.com/puniyu/core/compare/puniyu_task-v0.8.9...puniyu_task-v0.8.10)

### 🚜 重构


- *(core)* 移除未使用的log依赖并优化trait定义 - ([00df4ca](https://github.com/puniyu/core/commit/00df4cae4fad7f0e4e1fbf8798ff158019fc542b))




## [0.8.9](https://github.com/puniyu/core/compare/puniyu_task-v0.8.8...puniyu_task-v0.8.9)

### ⛰️ 新功能


- 引入AdapterHandle、BotHandle和CommandHandle统一资源管理 ([#286](https://github.com/puniyu/core/pull/286)) - ([baaeede](https://github.com/puniyu/core/commit/baaeede88979c150f4c97868226dd020aa10be3d))




## [0.8.8](https://github.com/puniyu/core/compare/puniyu_task-v0.8.7...puniyu_task-v0.8.8)

### 📚 文档


- 完善doc文档 - ([80f5206](https://github.com/puniyu/core/commit/80f5206397be9d4cdf85a8342361e3971a0331c2))




## [0.8.3](https://github.com/puniyu/core/compare/puniyu_task-v0.8.2...puniyu_task-v0.8.3)

### ⛰️ 新功能


- *(config)* 添加配置目录自动创建和错误日志记录 - ([3c144ce](https://github.com/puniyu/core/commit/3c144ced8136d5ca95a943b2ed929c220e2fde4c))



### 🎨 样式


- Format - ([625cf92](https://github.com/puniyu/core/commit/625cf9250625c35c19e0cedabd7fdde0a368ebfd))




## [0.8.2](https://github.com/puniyu/core/compare/puniyu_task-v0.8.1...puniyu_task-v0.8.2)

### ⚙️ 杂项


- Update Cargo.toml dependencies



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_task-v0.8.0...puniyu_task-v0.8.1)

### ⚙️ 杂项


- 更新项目配置 - ([548e4d9](https://github.com/puniyu/core/commit/548e4d9166f6bcb7b36d936e73015966e6927462))


## [0.8.0]

### ⛰️ 新功能


- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/core/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🚜 重构


- *(command)* 重构命令系统API并优化参数处理 - ([014c60c](https://github.com/puniyu/core/commit/014c60c1658ac6a3624cf7e44d05fe31329981eb))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 - ([c403f08](https://github.com/puniyu/core/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/core/pull/165)) - ([9482b34](https://github.com/puniyu/core/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/core/pull/150)) - ([e06459e](https://github.com/puniyu/core/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/core/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 使用jiff替换chrono库并优化时间处理 ([#179](https://github.com/puniyu/core/pull/179)) - ([8d0257a](https://github.com/puniyu/core/commit/8d0257a9663c43dceafa6e35f7e6f382425df727))
- 重构工作区crates ([#53](https://github.com/puniyu/core/pull/53)) - ([f55ab51](https://github.com/puniyu/core/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

