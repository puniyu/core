use actix_web::body::BoxBody;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[allow(dead_code)]
pub struct PrettyJson<T> {
	pub(crate) inner: T,
	pub(crate) status: StatusCode,
}

impl<T: Serialize> Responder for PrettyJson<T> {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		match serde_json::to_string_pretty(&self.inner) {
			Ok(body) => HttpResponse::build(self.status)
				.insert_header((header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()))
				.body(body),
			Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
		}
	}
}
