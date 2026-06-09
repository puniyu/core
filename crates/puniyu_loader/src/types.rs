use puniyu_adapter_core::Adapter;
use puniyu_plugin_core::Plugin;
use std::sync::Arc;

/// 加载器上下文
pub struct LoadContext {
	/// 应用名称
	pub app_name: &'static str,
	/// 当前工作目录
	pub cwd_dir: std::path::PathBuf,
}

/// 组件来源。
#[derive(Debug, Clone)]
pub enum ComponentSource {
	/// 内置组件
	Builtin,
	/// 从文件路径加载
	Path(std::path::PathBuf),
}

/// 元信息
#[derive(Debug, Clone)]
pub struct DiscoveryMeta {
	/// 组件来源
	pub source: ComponentSource,
	/// 优先级
	pub priority: i32,
}

/// 已发现的适配器。
pub struct DiscoveredAdapter {
	/// 适配器实例
	pub instance: Arc<dyn Adapter>,
	/// 发现元信息
	pub meta: DiscoveryMeta,
}

/// 已发现的插件。
pub struct DiscoveredPlugin {
	/// 插件实例
	pub instance: Arc<dyn Plugin>,
	/// 发现元信息
	pub meta: DiscoveryMeta,
}

/// 组件集合
pub struct ComponentSet {
	/// 发现的适配器列表
	pub adapters: Vec<DiscoveredAdapter>,
	/// 发现的插件列表
	pub plugins: Vec<DiscoveredPlugin>,
}
