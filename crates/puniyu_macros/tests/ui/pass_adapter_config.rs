use puniyu_macros::{adapter, adapter_config};
use puniyu_adapter::types::{adapter_info, AdapterInfo, AdapterPlatform, AdapterProtocol, SendMsgType};
use puniyu_adapter::{contact::ContactType, message::Message};
use std::sync::Arc;

struct Runtime {
	adapter: AdapterInfo,
}

impl puniyu_adapter::runtime::AdapterProvider for Runtime {
	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter
	}
}

#[puniyu_adapter::__private::async_trait]
impl puniyu_adapter::runtime::SendMessage for Runtime {
	async fn send_message(&self, _contact: &ContactType<'_>, _message: &Message) -> puniyu_adapter::Result<SendMsgType> {
		Ok(SendMsgType {
			message_id: "1".into(),
			time: 0,
		})
	}
}

fn runtime() -> Arc<dyn puniyu_adapter::runtime::AdapterRuntime> {
	Arc::new(Runtime {
		adapter: adapter_info!("test", AdapterPlatform::Other, AdapterProtocol::Console),
	})
}

#[adapter(runtime = runtime)]
async fn __main() -> puniyu_adapter::Result {
	Ok(())
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
#[adapter_config(name = "sample")]
struct SampleConfig {
	value: String,
}

fn main() {}
