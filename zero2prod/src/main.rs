//! zero2prod/src/main.rs
use std::io::Result;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
//? ********************************************************

type Void = ();
//? ********************************************************

async fn greet(req: HttpRequest) -> impl Responder {
	let name: &str = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}
//? ********************************************************

#[tokio::main]
async fn main() -> Result<Void> {
	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(greet))
			.route("/{name}", web::get().to(greet))
	})
		.bind("127.0.0.1:8000")?
		.run()
		.await
}
//? ********************************************************