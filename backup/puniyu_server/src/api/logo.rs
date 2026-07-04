use crate::get_logo;
use actix_web::{HttpResponse, Responder};

pub(super) async fn logo() -> impl Responder {
	let logo = get_logo();
	if logo.is_empty() {
		return HttpResponse::NotFound().finish();
	}
	let mime_type = infer::get(&logo)
		.map(|kind| kind.mime_type())
		.or_else(|| {
			std::str::from_utf8(&logo)
				.ok()
				.filter(|text| text.trim_start().starts_with("<svg"))
				.map(|_| mime::IMAGE_SVG.as_ref())
		})
		.unwrap_or(mime::APPLICATION_OCTET_STREAM.as_ref());

	HttpResponse::Ok().content_type(mime_type).body(logo)
}
