use puniyu_macros::{plugin, plugin_hook};
use puniyu_plugin::hook::HookType;

#[plugin]
async fn __main() {}

#[plugin_hook(name = "on_message", hook_type = "event.message", priority = 100)]
async fn on_message(_event: &HookType) -> puniyu_plugin::Result {
    Ok(())
}

fn main() {}
