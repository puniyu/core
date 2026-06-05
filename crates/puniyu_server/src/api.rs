use actix_web::web;

mod logo;

pub(crate) fn register_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("/logo").route(web::get().to(logo::logo)));
}
