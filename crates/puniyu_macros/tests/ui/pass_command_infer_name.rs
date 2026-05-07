use puniyu_macros::{command, plugin};
use puniyu_plugin::command::CommandAction;
use puniyu_context::MessageContext;

#[plugin]
async fn __main() {}

#[command(desc = "echo", alias = ["say"], permission = "all")]
#[arg(name = "message", desc = "消息")]
async fn echo_message(_ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
    CommandAction::done()
}

fn main() {}
