use actix_web::web::ServiceConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerFunction(Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>);

impl ServerFunction {
	pub fn new<F>(f: F) -> Self
	where
		F: Fn(&mut ServiceConfig) + Send + Sync + 'static,
	{
		Self(Arc::new(f))
	}

	pub fn call(&self, cfg: &mut ServiceConfig) {
		self.0(cfg)
	}
}

impl<F> From<F> for ServerFunction
where
	F: Fn(&mut ServiceConfig) + Send + Sync + 'static,
{
	fn from(f: F) -> Self {
		Self::new(f)
	}
}

impl std::ops::Deref for ServerFunction {
	type Target = dyn Fn(&mut ServiceConfig) + Send + Sync;

	fn deref(&self) -> &Self::Target {
		&*self.0
	}
}
