use puniyu_macros::{command, plugin};
use puniyu_plugin::command::CommandAction;
use puniyu_context::MessageContext;

#[plugin]
async fn __main() {}

#[command(name = "echo", desc = "echo", alias = ["say"], permission = "all")]
#[arg(name = "message", desc = "消息")]
#[arg(name = "count", r#type = "int", mode = "named", desc = "次数")]
async fn echo(_ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
    CommandAction::done()
}

fn main() {}
