use puniyu_macros::{plugin, task};

#[plugin]
async fn __main() {}

#[task(cron = "0 * * * * *", name = "health_check")]
async fn health_check() -> puniyu_plugin::Result {
    Ok(())
}

fn main() {}
