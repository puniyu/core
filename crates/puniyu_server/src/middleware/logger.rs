use crate::logger::info;
use actix_web::{
	Error,
	dev::{Service, ServiceRequest, ServiceResponse, Transform},
	http::header::HeaderMap,
};
use futures::future::LocalBoxFuture;
use std::net::IpAddr;
use std::{
	future::{Ready, ready},
	time::Instant,
};

pub struct AccessLog;

pub struct AccessLogMiddleware<S> {
	service: S,
}

impl<S, B> Transform<S, ServiceRequest> for AccessLog
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Transform = AccessLogMiddleware<S>;
	type InitError = ();
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(AccessLogMiddleware { service }))
	}
}

impl<S, B> Service<ServiceRequest> for AccessLogMiddleware<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	actix_web::dev::forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		let start = Instant::now();
		let method = req.method().clone();
		let path = req.path().to_string();
		let peer_addr = req.peer_addr();
		let ip_str = parse_ip(req.headers()).unwrap_or_else(|| {
			peer_addr.map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string())
		});

		let fut = self.service.call(req);

		Box::pin(async move {
			let response = fut.await?;
			let duration = start.elapsed();

			info!(
				"{} | {} | {} | {}ms | {}",
				method,
				path,
				response.status().as_u16(),
				duration.as_millis(),
				ip_str
			);

			Ok(response)
		})
	}
}

fn parse_ip(req: &HeaderMap) -> Option<String> {
	req.get("X-Forwarded-For").and_then(|hv| hv.to_str().ok()).and_then(|header_value| {
		header_value.split(',').map(|ip| ip.trim()).find_map(|ip| {
			ip.parse::<IpAddr>().ok().and_then(|addr| {
				if let IpAddr::V4(ipv4) = addr { Some(ipv4.to_string()) } else { None }
			})
		})
	})
}
