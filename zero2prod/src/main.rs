//! zero2prod/src/main.rs
use std::io::Result;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
//? ********************************************************

type Void = ();
//? ********************************************************
/// greet is an asynchronous function that takes an HttpRequest
/// as input and returns something that implements the Responder
/// trait 19 . A type implements the Responder trait if it can be
/// converted into a HttpResponse - it is implemented off the shelf
/// for a variety of common types (e.g. strings, status codes, bytes,
/// HttpResponse, etc.) and we can roll our own implementations if needed.
/// # Example
///
/// ```rust
/// use actix_web::{web, App, HttpServer, Responder};
///
/// async fn greet(name: web::Path<String>) -> impl Responder {
///     format!("Hello, {}!", name)
/// }
///
/// let app = || {
///     App::new()
///         .service(web::resource("/{name}").to(greet))
/// };
///
/// let server = HttpServer::new(app);
///
/// // Start the server
/// let addr = "127.0.0.1:8080";
/// server.bind(addr)?.run().await?;
/// ```
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