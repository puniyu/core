use puniyu_macros::{plugin, plugin_config};

#[plugin]
async fn __main() {}

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[plugin_config(name = "sample")]
struct SampleConfig {
	value: String,
}

fn main() {}
