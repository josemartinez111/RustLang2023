// !src/routes.rs
use actix_web::web;
// !The super keyword is used to refer to the parent module
// !of the current module, or files that live in the same mod.
use super::handlers::*;
//? ********************************************************

pub fn general_routes(configure_route: &mut web::ServiceConfig) {
	configure_route.route(
		"/health",
		web::get().to(health_check_handler),
	);
}
//? ********************************************************
//? ********************************************************