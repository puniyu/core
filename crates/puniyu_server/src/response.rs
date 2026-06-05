#![allow(dead_code)]

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::PrettyJson;

#[derive(Serialize, Deserialize)]
pub struct Response<T = ()> {
	pub code: u16,
	pub data: Option<T>,
	pub message: String,
	#[serde(skip)]
	#[allow(dead_code)]
	status: StatusCode,
}

impl<T: Serialize> Responder for Response<T> {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		HttpResponse::build(self.status).json(&self)
	}
}

impl Default for Response<()> {
	fn default() -> Self {
		Self::error(StatusCode::NOT_FOUND, "not found")
	}
}

#[allow(dead_code)]
impl<T> Response<T> {
	pub fn ok(message: impl Into<String>, data: Option<T>) -> Self {
		Self {
			code: StatusCode::OK.as_u16(),
			status: StatusCode::OK,
			data,
			message: message.into(),
		}
	}

	pub fn success(data: T) -> Self {
		Self::ok("success", Some(data))
	}

	pub fn error(status: StatusCode, message: impl Into<String>) -> Self {
		Self { code: status.as_u16(), status, data: None, message: message.into() }
	}

	pub fn not_found(message: impl Into<String>) -> Self {
		Self::error(StatusCode::NOT_FOUND, message)
	}

	pub fn bad_request(message: impl Into<String>) -> Self {
		Self::error(StatusCode::BAD_REQUEST, message)
	}

	pub fn internal_error(message: impl Into<String>) -> Self {
		Self::error(StatusCode::INTERNAL_SERVER_ERROR, message)
	}

	pub fn unauthorized(message: impl Into<String>) -> Self {
		Self::error(StatusCode::UNAUTHORIZED, message)
	}

	pub fn forbidden(message: impl Into<String>) -> Self {
		Self::error(StatusCode::FORBIDDEN, message)
	}
}

impl<T: Serialize> Response<T> {
	pub fn pretty(self) -> PrettyJson<Self> {
		let status = self.status;
		PrettyJson { inner: self, status }
	}
}
