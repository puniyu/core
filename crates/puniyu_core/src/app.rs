mod adapter;
mod config;
mod hook;
mod loader;
mod plugin;
mod server;

use bytes::Bytes;
use convert_case::{Case, Casing};
use puniyu_adapter_core::Adapter;
use puniyu_common::app::app_name;
use puniyu_handler::Handler;
use puniyu_hook::{HookType, StatusType};
use puniyu_loader::Loader;
use puniyu_plugin_core::Plugin;
use puniyu_version::Version;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use std::{env, io};
use tokio::{fs, signal};

use crate::logger::{core_debug, core_error, core_info};

const VERSION: Version = Version {
	major: const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u64),
	minor: const_str::parse!(env!("CARGO_PKG_VERSION_MINOR"), u64),
	patch: const_str::parse!(env!("CARGO_PKG_VERSION_PATCH"), u64),
};

/// 应用构建器
///
/// 使用构建器模式配置和创建应用实例
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_core::App;
///
/// let app = App::builder()
///     .with_app_name("my_bot")
///     .with_plugin(MyPlugin)
///     .with_adapter(MyAdapter)
///     .build()
///     .unwrap();
///
/// app.run().await?;
/// ```
pub struct AppBuilder {
	name: &'static str,
	version: &'static Version,
	logo: Option<Bytes>,
	working_dir: PathBuf,
	plugins: Vec<Arc<dyn Plugin>>,
	adapters: Vec<Arc<dyn Adapter>>,
	loaders: Vec<Arc<dyn Loader>>,
	handlers: Vec<Arc<dyn Handler>>,
	configs: Vec<Arc<dyn puniyu_config::Config>>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		#[allow(clippy::unwrap_used)]
		Self {
			name: "Core",
			version: &VERSION,
			logo: None,
			working_dir: std::env::current_dir().unwrap(),
			plugins: Vec::new(),
			adapters: Vec::new(),
			loaders: Vec::new(),
			handlers: Vec::new(),
			configs: Vec::new(),
		}
	}
}

impl AppBuilder {
	/// 设置应用名称
	///
	/// # 参数
	///
	/// - `name`: 应用名称
	pub fn with_app_name(mut self, name: &'static str) -> Self {
		self.name = name;
		self
	}

	/// 设置应用版本
	///
	/// # 参数
	///
	/// - `version`: 应用版本
	///
	pub fn with_app_version(mut self, version: &'static Version) -> Self {
		self.version = version;
		self
	}

	/// 设置应用 Logo
	///
	/// # 参数
	///
	/// - `logo`: Logo 图片的字节数据
	pub fn with_app_logo(mut self, logo: Bytes) -> Self {
		self.logo = Some(logo);
		self
	}

	/// 设置工作目录
	///
	/// # 参数
	///
	/// - `dir`: 工作目录路径
	pub fn with_working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.working_dir = dir.into();
		self
	}

	/// 添加插件
	///
	/// 接受任何实现了 `Plugin` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `plugin`: 插件实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_plugin(MyPlugin::new())
	///     .with_plugin(AnotherPlugin)
	/// ```
	pub fn with_plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
		self.plugins.push(Arc::new(plugin));
		self
	}

	/// 添加适配器
	///
	/// 接受任何实现了 `Adapter` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `adapter`: 适配器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_adapter(ConsoleAdapter::new())
	///     .with_adapter(HttpAdapter::new())
	/// ```
	pub fn with_adapter<A: Adapter + 'static>(mut self, adapter: A) -> Self {
		self.adapters.push(Arc::new(adapter));
		self
	}

	/// 添加加载器
	///
	/// 接受任何实现了 `Loader` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `loader`: 加载器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_loader(PluginLoader::new())
	/// ```
	pub fn with_loader<L: Loader + 'static>(mut self, loader: L) -> Self {
		self.loaders.push(Arc::new(loader));
		self
	}

	/// 添加处理器
	///
	/// 接受任何实现了 `Handler` trait 的类型，在编译期确定
	///
	/// # 参数
	///
	/// - `handler`: 处理器实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_handler(MyHandler::new())
	///     .with_handler(AnotherHandler)
	/// ```
	pub fn with_handler<H: Handler + 'static>(mut self, handler: H) -> Self {
		self.handlers.push(Arc::new(handler));
		self
	}

	/// 添加自定义配置
	///
	/// 接受任何实现了 `Config` trait 的类型
	///
	/// # 参数
	///
	/// - `config`: 配置实例
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// App::builder()
	///     .with_config(DatabaseConfig::default())
	/// ```
	pub fn with_config<C: puniyu_config::Config + Default + 'static>(mut self, config: C) -> Self {
		self.configs.push(Arc::new(config));
		self
	}

	/// 构建应用实例
	///
	/// # 返回值
	///
	/// 返回配置好的 `App` 实例
	pub fn build(self) -> App {
		App { inner: self }
	}
}

pub struct App {
	inner: AppBuilder,
}

impl App {
	pub fn builder() -> AppBuilder {
		AppBuilder::default()
	}
	/// 运行应用
	///
	/// 初始化所有组件并启动应用，直到接收到中断信号
	///
	/// # 返回值
	///
	/// 成功返回 `Ok(())`，失败返回 IO 错误
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let app = App::builder().build();
	/// app.run().await?;
	/// ```
	pub async fn run(self) -> io::Result<()> {
		use crate::common::format_duration;
		use puniyu_common::app::{AppInfo, set_app_info};
		use puniyu_common::uptime;
		use puniyu_loader::LoaderRegistry;
		use std::time::Duration;

		let start_time = Instant::now();
		let name = self.inner.name;
		let version = self.inner.version;
		let logo = self.inner.logo;
		let working_dir = self.inner.working_dir;
		let loaders = self.inner.loaders;
		let handlers = self.inner.handlers;
		let plugins = self.inner.plugins;
		let adapters = self.inner.adapters;
		let configs = self.inner.configs;
		use puniyu_path::{
			adapter_dir, app_dir, config_dir, data_dir, log_dir, plugin_dir, resource_dir,
		};

		let dirs = vec![
			app_dir(),
			adapter_dir(),
			data_dir(),
			config_dir(),
			resource_dir(),
			plugin_dir(),
			log_dir(),
			puniyu_path::plugin::config_dir(),
			puniyu_path::plugin::data_dir(),
			puniyu_path::plugin::resource_dir(),
			puniyu_path::plugin::temp_dir(),
			puniyu_path::adapter::config_dir(),
			puniyu_path::adapter::data_dir(),
			puniyu_path::adapter::resource_dir(),
			puniyu_path::adapter::temp_dir(),
		];
		for dir in dirs {
			fs::create_dir_all(&dir).await?;
		}
		let info = AppInfo::new(name, version, working_dir);
		set_app_info(info);
		
		if !configs.is_empty() {
			for config in configs {
				let config_name = config.name().to_string();
				let file_path = config.path().join(format!("{}.toml", &config_name));
				if let Err(e) = puniyu_config::ConfigRegistry::register_entry(
					&config_name,
					file_path,
					config.to_value(),
				) {
					core_error!("Failed to register config: {:?}", e);
				}
			}
		}

		puniyu_config::init();
		puniyu_task::init().await;

		#[cfg(feature = "log")]
		{
			crate::logger::log_init();
		}
		for handler in handlers.into_iter() {
			if let Err(e) = puniyu_handler::HandlerRegistry::register(handler) {
				core_error!("Failed to register handler: {}", e);
			}
		}
		for loader in loaders.into_iter() {
			if let Err(e) = LoaderRegistry::register(loader) {
				core_error!("Failed to register loader: {}", e);
			}
		}

		if let Err(e) = init_app(plugins, adapters, LoaderRegistry::all()).await {
			core_error!("Failed to init app: {}", e);
		}
		execute_hooks(StatusType::Start).await;

		let app_name = app_name().to_case(Case::Lower);

		if let Err(e) = puniyu_dispatch::EventEmitter::run() {
			core_error!("Failed to start event emitter: {}", e);
		}

		if let Some(logo) = logo {
			puniyu_server::set_logo(logo);
		}

		let config = puniyu_config::app_config();
		let config = config.server();
		let host = config.host();
		let port = config.port();
		let server_runtime = puniyu_server::start_server(host, port)?;

		let duration_str = format_duration(start_time.elapsed());
		core_info!(
			"{} initialized in {}",
			app_name.fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);

		signal::ctrl_c().await?;
		execute_hooks(StatusType::Stop).await;
		puniyu_dispatch::EventEmitter::stop();
		if let Err(e) = server_runtime.shutdown().await {
			core_error!("Server exited with error: {}", e);
		}
		core_info!(
			"{} uptime: {}",
			app_name.to_case(Case::Lower).fg_rgb::<64, 224, 208>(),
			format_duration(Duration::from_secs(uptime())).fg_rgb::<255, 127, 80>()
		);
		Ok(())
	}
}

async fn init_app(
	plugins: Vec<Arc<dyn Plugin>>,
	adapters: Vec<Arc<dyn Adapter>>,
	loaders: Vec<Arc<dyn Loader>>,
) -> io::Result<()> {
	core_debug!("adapter loading...");
	for adapter in adapters {
		if let Err(e) = adapter::init_adapter(adapter).await {
			core_error!("Failed to init adapter: {}", e);
		}
	}
	core_debug!("adapter loaded!");
	core_debug!("loader loading...");
	for loader in loaders {
		if let Err(e) = loader::init_loader(loader).await {
			core_error!("Failed to register loader: {}", e);
		}
	}
	core_debug!("loader loaded!");
	core_debug!("plugin loading...");
	for plugin in plugins {
		if let Err(e) = plugin::init_plugin(plugin).await {
			core_error!("Failed to init plugin: {}", e);
		}
	}
	core_debug!("plugin loaded!");
	core_info!("loaders: {}", puniyu_loader::LoaderRegistry::all().len());
	core_info!("adapters: {}", puniyu_adapter_core::AdapterRegistry::all().len());
	core_info!("plugins: {}", puniyu_plugin_core::PluginRegistry::all().len());
	core_info!("commands: {}", puniyu_command::CommandRegistry::all().len());
	core_info!("handlers: {}", puniyu_handler::HandlerRegistry::all().len());
	core_info!("hooks: {}", puniyu_hook::HookRegistry::all().len());
	Ok(())
}

async fn execute_hooks(status_type: StatusType) {
	use puniyu_hook::HookRegistry;
	let mut hooks = HookRegistry::all()
		.into_iter()
		.filter(|x| match x.builder.r#type() {
			HookType::Status(status) => status == &status_type,
			_ => false,
		})
		.collect::<Vec<_>>();
	hooks.sort_unstable_by_key(|a| a.builder.priority());

	for hook in hooks {
		if let Err(e) = hook.builder.execute(None).await {
			match status_type {
				StatusType::Start => core_error!("Failed to execute start hook: {}", e),
				StatusType::Stop => core_error!("Failed to execute stop hook: {}", e),
			}
		}
		if let Err(e) = HookRegistry::unregister(hook.source) {
			core_error!("Failed to unregister hook: {}", e);
		}
	}
}
