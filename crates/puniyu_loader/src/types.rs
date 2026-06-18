use puniyu_adapter_api::AdapterHandle;
use puniyu_adapter_core::Adapter;
use puniyu_plugin_core::PluginHandle;
use std::sync::Arc;

/// 加载器上下文
pub struct LoadContext {
	pub app_name: &'static str,
	pub cwd_dir: std::path::PathBuf,
}

/// 组件来源。
#[derive(Debug, Clone)]
pub enum ComponentSource {
	Builtin,
	Path(std::path::PathBuf),
}

/// 元信息
#[derive(Debug, Clone)]
pub struct DiscoveryMeta {
	pub source: ComponentSource,
	pub priority: i32,
}

/// 已发现的适配器。
pub struct DiscoveredAdapter {
	/// 适配器实例（完整 Adapter trait）
	pub adapter: Arc<dyn Adapter>,
	/// 适配器句柄（用于注册表和热重载）
	pub handle: AdapterHandle,
	/// 发现元信息
	pub meta: DiscoveryMeta,
}

/// 已发现的插件。
pub struct DiscoveredPlugin {
	pub instance: PluginHandle,
	pub meta: DiscoveryMeta,
}

/// 组件集合
pub struct Components {
	pub adapters: Vec<DiscoveredAdapter>,
	pub plugins: Vec<DiscoveredPlugin>,
}
