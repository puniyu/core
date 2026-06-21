use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::PrettyJson;

#[derive(Serialize, Deserialize)]
pub struct Response<T = ()> {
	pub(crate) inner: puniyu_common::Response<T>,
}

impl<T> std::ops::Deref for Response<T> {
	type Target = puniyu_common::Response<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}


impl<T: Serialize> Responder for Response<T> {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let status = StatusCode::from_u16(self.inner.code.as_u16())
			.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
		HttpResponse::build(status).json(&self.inner)
	}
}

impl<T: Serialize> Response<T> {
	pub fn pretty(self) -> PrettyJson<Self> {
		let code = StatusCode::from_u16(self.inner.code.as_u16())
			.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
		PrettyJson { inner: self, code }
	}
}
