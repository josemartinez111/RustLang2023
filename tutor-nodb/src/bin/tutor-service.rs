// !tutor-nodb/src/bin/tutor-service.rs
#[path="../handlers.rs"]
mod handlers;
#[path="../routes.rs"]
mod routes;
#[path="../state.rs"]
mod state;
#[path="../types.rs"]
mod types;

use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use crate::routes::*;
use crate::state::AppSate;
use crate::types::Void;
//? ********************************************************

#[actix_rt::main]
async fn main() -> io::Result<Void> {
	// *Initialize application state
	let shared_data: Data<AppSate> = web::Data::new(AppSate {
		health_check_response: "I'm good. You've already asked me ".to_string(),
		visit_count: Mutex::new(0),
	});
	// *Initialize Actix web server with the web application,
	// *listen on port 3000 and run the server
	let app = move || {
		App::new()
			.app_data(shared_data.clone())
		.configure(general_routes)
	};
	
	HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
	
}
//? ********************************************************


//? ********************************************************
