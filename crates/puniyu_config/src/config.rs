use puniyu_common::read_config;
use puniyu_path::config_dir;
use std::thread;
use std::time::Duration;
use sugar_path::SugarPath;

use crate::logger::{config_debug, config_error, config_info};

/// 启动配置目录监听线程
pub fn start_config_watcher() {
	use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
	thread::spawn(|| {
		config_debug!("Configuration file watcher started");

		let mut debouncer =
			new_debouncer(Duration::from_secs(2), None, |res: DebounceEventResult| match res {
				Ok(events) => {
					for event in events.iter() {
						if !matches!(
							event.event.kind,
							notify::EventKind::Modify(_)
								| notify::EventKind::Create(_)
								| notify::EventKind::Remove(_)
						) {
							continue;
						}

						for path in event.event.paths.iter() {
							match std::env::current_dir() {
								Ok(cwd) => config_info!(
									"File changed: {}",
									path.relative(&cwd).to_slash_lossy()
								),
								Err(_) => config_info!("File changed: {}", path.to_slash_lossy()),
							}

							use crate::ConfigRegistry;
							if ConfigRegistry::all()
								.iter()
								.any(|c| c.path.to_slash_lossy() == *path.to_slash_lossy())
								&& let Some(name) = path.file_stem().and_then(|s| s.to_str())
								&& let Some(dir) = path.parent()
								&& let Ok(value) = read_config::<toml::Value>(dir, name)
							{
								if let Err(e) = ConfigRegistry::update(path.as_path(), value) {
									config_error!("Failed to update config: {}, {}", name, e);
								} else {
									config_info!("Config updated: {}", name);
								}
							}
						}
					}
				}
				Err(e) => config_error!("Watch error: {:?}", e),
			})
			.expect("Failed to create config file watcher");

		debouncer
			.watch(config_dir().as_path(), notify::RecursiveMode::Recursive)
			.expect("Failed to start watching config directory");

		thread::park();
	});
}
