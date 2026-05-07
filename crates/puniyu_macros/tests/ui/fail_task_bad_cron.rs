use puniyu_macros::task;

#[task(cron = "bad cron")]
async fn health_check() -> puniyu_plugin::Result {
    Ok(())
}

fn main() {}
