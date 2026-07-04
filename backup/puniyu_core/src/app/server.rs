use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_server::ServerFunction;

pub fn init_server(source: SourceType, server: ServerFunction) -> Result<(), Error> {
	use puniyu_server::ServerRegistry;
	ServerRegistry::register(source, server)?;
	Ok(())
}
