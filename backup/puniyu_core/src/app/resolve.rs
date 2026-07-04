use puniyu_loader::*;

pub(crate) struct ResolvedComponents {
	pub adapters: Vec<DiscoveredAdapter>,
	pub plugins: Vec<DiscoveredPlugin>,
}

impl ResolvedComponents {
	pub fn add_adapter(&mut self, a: DiscoveredAdapter) {
		let name = a.handle.get().adapter_info().name.to_string();
		if self.adapters.iter().any(|x| x.handle.get().adapter_info().name == name) {
			puniyu_common::core_warn!(
				"duplicate adapter detected — keeping higher priority occurrence"
			);
			return;
		}
		self.adapters.push(a);
	}

	pub fn add_plugin(&mut self, p: DiscoveredPlugin) {
		let name = p.handle.get().name().to_string();
		if self.plugins.iter().any(|x| x.handle.get().name() == name) {
			puniyu_common::core_warn!(
				"duplicate plugin detected — keeping higher priority occurrence"
			);
			return;
		}
		self.plugins.push(p);
	}

	pub fn sort(&mut self) {
		self.adapters.sort_by_key(|b| std::cmp::Reverse(b.meta.priority));
		self.plugins.sort_by_key(|b| std::cmp::Reverse(b.meta.priority));
	}
}

pub(crate) fn resolve(components: Vec<Components>) -> puniyu_error::Result<ResolvedComponents> {
	let config = puniyu_config::app_config();
	let adapter_config = config.adapter();
	let plugin_config = config.plugin();

	let mut resolved = ResolvedComponents { adapters: Vec::new(), plugins: Vec::new() };

	for component in components {
		for adapter in component.adapters {
			let name = adapter.handle.get().adapter_info().name.to_string();
			if is_enabled(&name, &adapter_config.enable_list(), &adapter_config.disable_list()) {
				resolved.add_adapter(adapter);
			} else {
				puniyu_common::core_warn!("adapter {} is disabled, skipping", name);
			}
		}
		for plugin in component.plugins {
			let name = plugin.handle.get().name().to_string();
			if is_enabled(&name, &plugin_config.enable_list(), &plugin_config.disable_list()) {
				resolved.add_plugin(plugin);
			} else {
				puniyu_common::core_warn!("plugin {} is disabled, skipping", name);
			}
		}
	}

	resolved.sort();

	Ok(resolved)
}

fn is_enabled(name: &str, enable_list: &[&str], disable_list: &[&str]) -> bool {
	if disable_list.contains(&name) {
		return false;
	}
	if enable_list.is_empty() && disable_list.is_empty() {
		return true;
	}
	if enable_list.is_empty() {
		return true;
	}
	enable_list.contains(&name)
}
