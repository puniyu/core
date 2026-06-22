use crate::{api, middleware};
use actix_web::dev::ServerHandle;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer, web};
use puniyu_common::app::app_name;

use crate::logger::server_info;
use std::io;
use std::net::IpAddr;
use std::sync::{LazyLock, Mutex};
use tokio::task::JoinHandle;

struct ServerControl {
	handle: ServerHandle,
	join_handle: JoinHandle<io::Result<()>>,
}

static SERVER_CONTROL: LazyLock<Mutex<Option<ServerControl>>> = LazyLock::new(|| Mutex::new(None));
pub fn start_server(host: IpAddr, port: u16) -> io::Result<()> {
	{
		let guard = SERVER_CONTROL.lock().map_err(|e| io::Error::other(e.to_string()))?;
		if guard.is_some() {
			return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Server already running"));
		}
	}

	server_info!("Server running on {}:{}", host, port);

	let server = HttpServer::new(|| {
		let app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(web::resource("/").to(|| async { format!("welcome {}", app_name()) }))
			.configure(|cfg| {
				cfg.service(web::scope("/api/v1").configure(api::register_routes));
			});

		#[cfg(feature = "registry")]
		let app = {
			use crate::registry::ServerRegistry;
			ServerRegistry::all().into_iter().fold(app, |app, cfg| {
				let builder = cfg.builder.clone();
				app.configure(move |sc| builder.call(sc))
			})
		};

		app
	})
	.bind((host, port))?;
	let running_server = server.run();
	let handle = running_server.handle();

	let join_handle = tokio::spawn(running_server);
	let control = ServerControl { handle, join_handle };

	SERVER_CONTROL.lock().map_err(|e| io::Error::other(e.to_string()))?.replace(control);
	Ok(())
}

pub async fn run_server(host: IpAddr, port: u16) -> io::Result<()> {
	start_server(host, port)?;
	shutdown_server().await
}

pub async fn stop_server() -> io::Result<()> {
	let control = SERVER_CONTROL
		.lock()
		.map_err(|e| io::Error::other(e.to_string()))?
		.take()
		.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Server not running"))?;

	control.handle.stop(true).await;
	Ok(())
}

pub async fn shutdown_server() -> io::Result<()> {
	let control = SERVER_CONTROL
		.lock()
		.map_err(|e| io::Error::other(e.to_string()))?
		.take()
		.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Server not running"))?;

	control.handle.stop(true).await;
	control
		.join_handle
		.await
		.map_err(|e| io::Error::other(format!("Server task join error: {}", e)))?
}

pub async fn restart_server(host: IpAddr, port: u16) -> io::Result<()> {
	stop_server().await?;
	start_server(host, port)
}
