# 变更日志

## [0.9.3](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.9.2...puniyu_loader_builtin-v0.9.3)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_plugin_core, puniyu_loader



## [0.9.2](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.9.1...puniyu_loader_builtin-v0.9.2)

### ⚙️ 杂项


- Updated the following local packages: puniyu_plugin_core, puniyu_loader



## [0.9.1](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.9.0...puniyu_loader_builtin-v0.9.1)

### ⛰️ 新功能


- 引入AdapterHandle、BotHandle和CommandHandle统一资源管理 ([#286](https://github.com/puniyu/core/pull/286)) - ([baaeede](https://github.com/puniyu/core/commit/baaeede88979c150f4c97868226dd020aa10be3d))



### 🚜 重构


- *(loader)* 移除DiscoveryMeta中的loader_name字段 - ([050ef9d](https://github.com/puniyu/core/commit/050ef9dab2375c8c40b6fa8a7a52214ed2c250c1))




## [0.9.0](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.18...puniyu_loader_builtin-v0.9.0)

### ⚙️ 杂项


- [**breaking**] Release v0.9 - ([6a715c7](https://github.com/puniyu/core/commit/6a715c78e0807dc300f4ec8de6d8e234183d17c4))




## [0.8.18](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.17...puniyu_loader_builtin-v0.8.18)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_plugin_core, puniyu_loader



## [0.8.17](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.16...puniyu_loader_builtin-v0.8.17)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_plugin_core, puniyu_loader



## [0.8.16](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.15...puniyu_loader_builtin-v0.8.16)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_plugin_core, puniyu_loader



## [0.8.15](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.14...puniyu_loader_builtin-v0.8.15)

### ⚙️ 杂项


- Updated the following local packages: puniyu_adapter_core, puniyu_plugin_core, puniyu_loader



## [0.8.14](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.13...puniyu_loader_builtin-v0.8.14)

### 🚜 重构


- *(puniyu_loader)* 移除不再使用的组件源类型 - ([0689984](https://github.com/puniyu/core/commit/068998405b37f7bfbbf17726e4144df342fe60d8))




## [0.8.13](https://github.com/puniyu/core/compare/puniyu_loader_builtin-v0.8.12...puniyu_loader_builtin-v0.8.13)

### ⚙️ 杂项


- *(puniyu_core)* 引入内置加载器和三层组件管理架构 - ([b6c5a01](https://github.com/puniyu/core/commit/b6c5a0116f7e34832d20cb7a059d7ef9944aa616))




## [0.8.12]

### 🚜 重构


- 新增 BuiltinLoader，支持构建器模式（new → with_adapter → with_plugin → discover）
