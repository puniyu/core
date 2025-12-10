
mod bot;

use actix_web::web::{ServiceConfig, self};
use puniyu_plugin::prelude::*;

#[server]
fn server(cfg: &mut ServiceConfig) {
	let api_routes = |cfg: &mut ServiceConfig| {
		cfg.route("/{bot_id}", web::get().to(bot::ws_handler));
		cfg.route("/{bot_id}/ws", web::get().to(bot::ws_handler));
	};
	cfg.service(
		web::scope("/api/bot")
			.configure(api_routes),
	);
}
