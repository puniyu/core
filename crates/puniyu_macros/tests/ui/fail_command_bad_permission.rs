use puniyu_macros::command;
use puniyu_plugin::command::CommandAction;
use puniyu_context::MessageContext;

#[command(name = "echo", permission = "root")]
async fn echo(_ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
    CommandAction::done()
}

fn main() {}
