// views/mod.rs
//? ********************************************************

use actix_web::web::ServiceConfig;
use crate::views::auth::auth_views_factory;

mod auth;
//? ********************************************************

pub fn views_factory(app: &mut ServiceConfig) {
	auth_views_factory(app);
}
//? ********************************************************
