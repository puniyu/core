use puniyu_macros::plugin_hook;
use puniyu_plugin::hook::HookType;

#[plugin_hook(hook_type = "event.unknown")]
async fn on_message(_event: &HookType) -> puniyu_plugin::Result {
    Ok(())
}

fn main() {}
