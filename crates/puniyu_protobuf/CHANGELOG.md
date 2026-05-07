# 变更日志

## [0.8.2](https://github.com/puniyu/core/compare/puniyu_protobuf-v0.8.1...puniyu_protobuf-v0.8.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_types, puniyu_bot



## [0.8.1](https://github.com/puniyu/core/compare/puniyu_protobuf-v0.8.0...puniyu_protobuf-v0.8.1)

### ⚙️ 杂项


- Updated the following local packages: puniyu_account, puniyu_contact, puniyu_element, puniyu_sender, puniyu_version, puniyu_adapter_types, puniyu_bot - ([0000000](https://github.com/puniyu/core/commit/0000000))


## [0.8.0]

### ⛰️ 新功能


- *(puniyu_event)* 添加通知和请求事件类型支持 - ([d7421d5](https://github.com/puniyu/core/commit/d7421d535c6ea247e6539372e34fda5fa26bb55b))
- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/core/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/core/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/core/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/core/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(event)* 移除SubEventType的Copy派生并优化clone实现 - ([f7a07f0](https://github.com/puniyu/core/commit/f7a07f0aa62b9ebea7fc9a268dc8c09a3a6344d9))
- *(protobuf)* 重构protobuf枚举命名和宏定义 - ([93fce77](https://github.com/puniyu/core/commit/93fce77a021a8e5c77786916b4839eaad70818d6))
- 重构事件模块 ([#180](https://github.com/puniyu/core/pull/180)) - ([bfbb9a7](https://github.com/puniyu/core/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/core/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/core/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/core/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 初始化仓库 - ([51af186](https://github.com/puniyu/core/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

## [0.2.4](https://github.com/puniyu/core/compare/puniyu_probuff-v0.2.3...puniyu_probuff-v0.2.4) - 2026-01-12


### ⚙️ 杂项



- Updated the following local packages: puniyu_types - ([0000000](https://github.com/puniyu/core/commit/0000000))

## [0.2.3](https://github.com/puniyu/core/compare/puniyu_probuff-v0.2.2...puniyu_probuff-v0.2.3) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types - ([0000000](https://github.com/puniyu/core/commit/0000000))

## [0.2.2](https://github.com/puniyu/core/compare/puniyu_probuff-v0.2.1...puniyu_probuff-v0.2.2) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/core/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/core/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/core/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))



### 贡献者

* @wuliya336
