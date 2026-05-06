# 变更日志

## [0.8.1](https://github.com/puniyu/puniyu/compare/puniyu_api-v0.8.0...puniyu_api-v0.8.1)

### ⛰️ 新功能


- *(puniyu_ipc)* 实现基于MessagePack的IPC协议 - ([76a2ed5](https://github.com/puniyu/puniyu/commit/76a2ed53d2e26fe44812e2ffdf9912fc81b41663))
- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/puniyu/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))
- 添加IPC通信模块和插件加载器 - ([2772943](https://github.com/puniyu/puniyu/commit/277294358132256cff352dc974dd1909e8995221))

### 🐛 Bug 修复


- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/puniyu/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/puniyu/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/puniyu/pull/165)) - ([9482b34](https://github.com/puniyu/puniyu/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/puniyu/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- 重构事件模块 ([#180](https://github.com/puniyu/puniyu/pull/180)) - ([bfbb9a7](https://github.com/puniyu/puniyu/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/puniyu/pull/178)) - ([fe74041](https://github.com/puniyu/puniyu/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(puniyu_plugin_core)* 添加包描述信息 - ([6632ade](https://github.com/puniyu/puniyu/commit/6632ade7e2e7f2fad01fc7127ce6cb05d22fb89a))
- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 初始化仓库 - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


## [0.8.0]

### ⛰️ 新功能


- *(runtime)* 添加ServerRuntime句柄管理服务器生命周期 - ([ff7ea10](https://github.com/puniyu/puniyu/commit/ff7ea10438a46fed01539d087fb50f83867fd0c3))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(bot)* 移除自定义ConsoleBot结构体并简化类型引用 - ([6ef97db](https://github.com/puniyu/puniyu/commit/6ef97dba212915cbd81d00f8eb06559254dfd195))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/puniyu/pull/165)) - ([9482b34](https://github.com/puniyu/puniyu/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- 重构事件模块 ([#180](https://github.com/puniyu/puniyu/pull/180)) - ([bfbb9a7](https://github.com/puniyu/puniyu/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/puniyu/pull/178)) - ([fe74041](https://github.com/puniyu/puniyu/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 初始化仓库 - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

