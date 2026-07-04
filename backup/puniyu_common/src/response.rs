use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T = ()> {
	#[serde(serialize_with = "serialize_status", deserialize_with = "deserialize_status")]
	pub code: StatusCode,
	pub data: Option<T>,
	pub message: String,
}

fn serialize_status<S: serde::Serializer>(status: &StatusCode, s: S) -> Result<S::Ok, S::Error> {
	s.serialize_u16(status.as_u16())
}

fn deserialize_status<'de, D: serde::Deserializer<'de>>(d: D) -> Result<StatusCode, D::Error> {
	let code = u16::deserialize(d)?;
	StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}

impl Default for Response<()> {
	fn default() -> Self {
		Self::error(StatusCode::NOT_FOUND, "not found")
	}
}


impl<T> Response<T> {
	pub fn ok(message: impl Into<String>, data: Option<T>) -> Self {
		Self {
			code: StatusCode::OK,
			data,
			message: message.into(),
		}
	}

	pub fn success(data: T) -> Self {
		Self::ok("success", Some(data))
	}

	pub fn error(code: StatusCode, message: impl Into<String>) -> Self {
		Self { code, data: None, message: message.into() }
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

