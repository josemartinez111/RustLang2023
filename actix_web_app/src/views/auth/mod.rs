// views/auth/mod.rs
//? ********************************************************

use actix_web::web::{get, scope, ServiceConfig};

mod login;
mod logout;
//? ********************************************************

// ! ServiceConfig
// * enables parts of app configuration to be declared separately
// * from the app itself. Helpful for modularizing large applications
pub fn auth_views_factory(app: &mut ServiceConfig) {
	// ? Register HTTP service factory
	app.service(
		scope("v1/auth")
			.route("login", get().to(login::login))
			.route("logout", get().to(logout::logout))
	);
}
//? ********************************************************
