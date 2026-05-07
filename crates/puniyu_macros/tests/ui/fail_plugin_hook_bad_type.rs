use puniyu_macros::{plugin, plugin_hook};
use puniyu_plugin::hook::HookType;

#[plugin]
async fn __main() {}

#[plugin_hook(hook_type = "event.unknown")]
async fn on_message(_event: &HookType) -> puniyu_plugin::Result {
    Ok(())
}

fn main() {}
