mod config;
mod install;
mod resolve;
mod server;

use bytes::Bytes;
use convert_case::{Case, Casing};
use puniyu_common::app::app_name;
use puniyu_handler::Handler;
use puniyu_loader::{LoadContext, Loader};
use puniyu_semver::Version;
use puniyu_version::VERSION;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::{fs, signal};

use puniyu_common::{core_debug, core_error, core_info};

type AsyncFn =
	Box<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;

/// 应用构建器
///
/// 使用构建器模式配置和创建应用实例。
/// 组件（adapter/plugin）必须通过 Loader（如 BuiltinLoader）传入，不再直接接受。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_core::App;
/// use puniyu_loader_builtin::BuiltinLoader;
///
/// let app = App::builder()
///     .with_app_name("my_bot")
///     .with_loader(BuiltinLoader::new()
///         .with_adapter(MyAdapter)
///         .with_plugin(MyPlugin))
///     .build();
///
/// app.run().await.unwrap();
/// ```
pub struct AppBuilder {
	name: &'static str,
	version: &'static Version,
	logo: Option<Bytes>,
	cwd_dir: PathBuf,
	loaders: Vec<Arc<dyn Loader>>,
	handlers: Vec<Arc<dyn puniyu_handler::Handler>>,
	configs: Vec<Arc<dyn puniyu_config::Config>>,
	on_start: Option<AsyncFn>,
	on_exit: Option<AsyncFn>,
}

impl Default for AppBuilder {
	fn default() -> Self {
		#[allow(clippy::unwrap_used)]
		Self {
			name: "Core",
			version: &VERSION,
			logo: None,
			cwd_dir: std::env::current_dir().unwrap(),
			loaders: Vec::new(),
			handlers: Vec::new(),
			configs: Vec::new(),
			on_start: None,
			on_exit: None,
		}
	}
}

impl AppBuilder {
	/// 设置应用名称
	pub fn with_app_name(mut self, name: &'static str) -> Self {
		self.name = name;
		self
	}

	/// 设置应用版本
	pub fn with_app_version(mut self, version: &'static Version) -> Self {
		self.version = version;
		self
	}

	/// 设置应用 Logo
	pub fn with_app_logo(mut self, logo: Bytes) -> Self {
		self.logo = Some(logo);
		self
	}

	/// 设置工作目录
	pub fn with_cwd_dir(mut self, dir: impl Into<PathBuf>) -> Self {
		self.cwd_dir = dir.into();
		self
	}

	/// 添加加载器
	///
	/// 所有 adapter 和 plugin 都必须通过 Loader 传入。
	/// 推荐使用 `puniyu_loader_builtin::BuiltinLoader` 携带编译期组件。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_loader_builtin::BuiltinLoader;
	///
	/// App::builder()
	///     .with_loader(
	///         BuiltinLoader::new()
	///             .with_adapter(MyAdapter)
	///             .with_plugin(MyPlugin)
	///     )
	/// ```
	pub fn with_loader<L: Loader + 'static>(mut self, loader: L) -> Self {
		self.loaders.push(Arc::new(loader));
		self
	}

	/// 添加处理器
	pub fn with_handler<H: Handler + 'static>(mut self, handler: H) -> Self {
		self.handlers.push(Arc::new(handler));
		self
	}

	/// 添加自定义配置
	pub fn with_config<C: puniyu_config::Config + Default + 'static>(mut self, config: C) -> Self {
		self.configs.push(Arc::new(config));
		self
	}

	/// 设置应用启动时的回调
	pub fn with_on_start<F, Fut>(mut self, f: F) -> Self
	where
		F: Fn() -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = ()> + Send + 'static,
	{
		self.on_start = Some(Box::new(move || Box::pin(f())));
		self
	}

	/// 设置应用退出时的回调
	pub fn with_on_exit<F, Fut>(mut self, f: F) -> Self
	where
		F: Fn() -> Fut + Send + Sync + 'static,
		Fut: std::future::Future<Output = ()> + Send + 'static,
	{
		self.on_exit = Some(Box::new(move || Box::pin(f())));
		self
	}

	/// 构建应用实例
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

	pub async fn run(self) -> io::Result<()> {
		use crate::common::format_duration;
		use puniyu_common::app::{AppInfo, set_app_info};
		use puniyu_common::uptime;
		use std::time::Duration;

		let start_time = Instant::now();
		let name = self.inner.name;
		let version = self.inner.version;
		let logo = self.inner.logo;
		let working_dir = self.inner.cwd_dir;
		let loaders = self.inner.loaders;
		let handlers = self.inner.handlers;
		let configs = self.inner.configs;
		let on_start = self.inner.on_start;
		let on_exit = self.inner.on_exit;

		let info = AppInfo::new(name, version, working_dir.clone());
		set_app_info(info);
		{
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
		}
		if let Some(callback) = on_start {
			(callback)().await;
		}

		puniyu_config::init();
		if !configs.is_empty() {
			for config in configs {
				let config_name = config.name().to_string();
				let file_path = config.path().join(format!("{}.toml", &config_name));
				let _ = puniyu_config::ConfigRegistry::register_entry(
					&config_name,
					file_path,
					config.to_value(),
				);
			}
		}

		puniyu_task::init().await;

		for handler in handlers.into_iter() {
			if let Err(e) = puniyu_handler::HandlerRegistry::register(handler) {
				core_error!("Failed to register handler: {}", e);
			}
		}

		core_debug!("discovering components...");
		let load_ctx = LoadContext { app_name: name, cwd_dir: working_dir };

		let mut loader_task = tokio::task::JoinSet::new();
		let load_ctx = Arc::new(load_ctx);
		for loader in loaders.into_iter() {
			core_debug!("discovering from loader: {}", loader.name());
			let ctx = Arc::clone(&load_ctx);
			loader_task.spawn(async move {
				match loader.discover(&ctx).await {
					Ok(set) => Some(set),
					Err(e) => {
						core_error!("Loader {} discover failed: {}", loader.name(), e);
						None
					}
				}
			});
		}
		let all_sets: Vec<_> = loader_task.join_all().await.into_iter().flatten().collect();

		let resolved = resolve::resolve(all_sets)
			.map_err(|e| std::io::Error::other(format!("Failed to resolve components: {}", e)))?;

		install::install(resolved).await?;

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
		puniyu_server::start_server(host, port)?;

		let duration_str = format_duration(start_time.elapsed());
		core_info!(
			"{} initialized in {}",
			app_name.fg_rgb::<64, 224, 208>(),
			duration_str.fg_rgb::<255, 127, 80>()
		);

		signal::ctrl_c().await?;

		if let Some(callback) = on_exit {
			(callback)().await;
		}

		puniyu_dispatch::EventEmitter::stop();
		if let Err(e) = puniyu_server::shutdown_server().await {
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
