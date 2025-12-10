# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/server-v0.6.0...server-v0.7.0) (2025-12-10)


### ♻️ 代码重构

* **puniyu_common:** 添加 stable 特性并重构版本信息模块 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_core:** 移除重复的 version 模块引用 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_server:** 优化 API 路由结构与响应类型泛型化 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **server:** 将服务器命令通道从 tokio mpsc 迁移至 flume ([1345cae](https://github.com/puniyu/puniyu/commit/1345caefb4b07f094d00521fd9286eb6b33b9ab5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.6.0 to 0.7.0
    * puniyu_types bumped from 0.6.0 to 0.7.0
    * puniyu_registry bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/server-v0.5.12...server-v0.6.0) (2025-12-02)


### ✨ 新功能

* **cli:** 引入命令行参数解析功能 ([11e3137](https://github.com/puniyu/puniyu/commit/11e31372aca53c35f15e8cab8b3067af353d25a7))
* **server:** 实现服务器控制功能并完善插件卸载功能 ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))


### 📝 文档更新

* **puniyu_library:** 更新 README 文档并优化库重载逻辑 ([f561597](https://github.com/puniyu/puniyu/commit/f561597ee8a829f648cc52a91f0bfe717b218449))


### 🔧 其他更新

* **puniyu_logger:** 更新 puniyu_logger 依赖版本 ([f949ad4](https://github.com/puniyu/puniyu/commit/f949ad40d00b11b9caa3f78fecb35fc37055742f))


### ♻️ 代码重构

* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.12 to 0.6.0
    * puniyu_types bumped from 0.5.12 to 0.6.0
    * puniyu_registry bumped from 0.5.12 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/server-v0.5.11...server-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.11 to 0.5.12
    * puniyu_types bumped from 0.5.11 to 0.5.12
    * puniyu_registry bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/server-v0.5.10...server-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.10 to 0.5.11
    * puniyu_types bumped from 0.5.10 to 0.5.11
    * puniyu_registry bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/server-v0.5.9...server-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.9 to 0.5.10
    * puniyu_types bumped from 0.5.9 to 0.5.10
    * puniyu_registry bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/server-v0.5.8...server-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.8 to 0.5.9
    * puniyu_types bumped from 0.5.8 to 0.5.9
    * puniyu_registry bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/server-v0.5.7...server-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.7 to 0.5.8
    * puniyu_types bumped from 0.5.7 to 0.5.8
    * puniyu_registry bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/server-v0.5.6...server-v0.5.7) (2025-11-23)


### 🔧 其他更新

* release main ([45cd505](https://github.com/puniyu/puniyu/commit/45cd5054d26007f5a499cf3a0329f1e0435b1deb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.6 to 0.5.7
    * puniyu_types bumped from 0.5.6 to 0.5.7
    * puniyu_registry bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/server-v0.5.6...server-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6
    * puniyu_types bumped from 0.5.4 to 0.5.6
    * puniyu_registry bumped from 0.5.5 to 0.5.6

## [0.5.6](https://github.com/puniyu/puniyu/compare/server-v0.5.5...server-v0.5.6) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.4 to 0.5.5

## [0.5.5](https://github.com/puniyu/puniyu/compare/server-v0.5.4...server-v0.5.5) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.3 to 0.5.4

## [0.5.4](https://github.com/puniyu/puniyu/compare/server-v0.5.2...server-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **core:** 支持设置app_logo ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
* **server:** 引入基础响应结构并优化logo接口 ([4b25653](https://github.com/puniyu/puniyu/commit/4b2565346293b18fff8490b7f28cec5579709565))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.1 to 0.5.4
    * puniyu_types bumped from 0.5.1 to 0.5.4
    * puniyu_registry bumped from 0.5.2 to 0.5.3

## [0.5.2](https://github.com/puniyu/puniyu/compare/server-v0.5.1...server-v0.5.2) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.1 to 0.5.2

## [0.5.1](https://github.com/puniyu/puniyu/compare/server-v0.5.0...server-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **server:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.0 to 0.5.1
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_registry bumped from 0.5.0 to 0.5.1

## 0.5.0 (2025-11-23)


### 🔧 其他更新

* **deps:** 更新 tokio 依赖并启用完整特性 ([1b717fa](https://github.com/puniyu/puniyu/commit/1b717faacc17a23de1ec0a7856744c80d97b1a85))


### ♻️ 代码重构

* **adapter:** 优化 Avatar 类型实现并添加服务器 logo 接口 ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
* **api:** 简化logo路由注册方式 ([4337b56](https://github.com/puniyu/puniyu/commit/4337b569d4abf62f53366710e19c0f049f898938))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_registry bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/server-v0.4.0...server-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **server:** Synchronize puniyu versions

## [0.4.0](https://github.com/puniyu/puniyu/compare/server-v0.2.0...server-v0.4.0) (2025-11-16)


### 🔧 其他更新

* **server:** Synchronize puniyu versions

## [0.2.0](https://github.com/puniyu/puniyu/compare/server-v0.1.8...server-v0.2.0) (2025-11-15)


### ✨ 新功能

* **server:** 条件编译控制日志初始化 ([859c86e](https://github.com/puniyu/puniyu/commit/859c86e8d0e2047643b7c41931055950ab96c839))
* **server:** 添加 HTTP 服务器模块并初始化配置 ([7e83259](https://github.com/puniyu/puniyu/commit/7e832595521a21081530a8c9c318d416e747ee0b))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([fd3d0e8](https://github.com/puniyu/puniyu/commit/fd3d0e851964cc27af11c500e2c5b0d6214a7bd0))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **server:** 移除日志功能的特性标志并初始化主函数中的日志 ([156fd0e](https://github.com/puniyu/puniyu/commit/156fd0e3dbf637d6dd5ee0630e68dd9dfbd27744))
