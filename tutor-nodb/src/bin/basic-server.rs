// !tutor-nodb/src/bin/basic-server.rs
use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
//? ********************************************************

type Void = ();
//? ********************************************************

// ?Configure routes
pub fn general_routes(route_config: &mut web::ServiceConfig) -> Void {
	route_config.route("/health", web::get()
		.to(health_check_handler),
	);
}

// ?Configure handler
pub async fn health_check_handler() -> impl Responder {
	HttpResponse::Ok().json("Hola, esto es una prueba de EzyTutor")
}
//? ********************************************************

#[actix_rt::main]
async fn main() -> io::Result<Void> {
	// ?Construct app and configure routes
	let app = move || App::new().configure(general_routes);
	
	// Start Http server
	HttpServer::new(app)
		.bind("127.0.0.1:3000")?
		.run()
		.await
}
//? ********************************************************
















