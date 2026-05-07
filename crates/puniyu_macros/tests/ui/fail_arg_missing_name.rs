use puniyu_macros::{arg, command, plugin};
use puniyu_plugin::command::CommandAction;
use puniyu_context::MessageContext;

#[plugin]
async fn __main() {}

#[arg(desc = "消息")]
#[command(name = "echo")]
async fn echo(_ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
	CommandAction::done()
}

fn main() {}
