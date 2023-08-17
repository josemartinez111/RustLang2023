//? ../actix_web_app/src/main.rs
mod views;
//? ********************************************************

use std::io;
use actix_web::{App, HttpServer};
//? ********************************************************

#[actix_web::main]
async fn main() -> io::Result<()> {
	let server_address = format!("127.0.0.1:{}", 8080);
	
	// Create the HTTP server
	let server_result = HttpServer::new(|| {
		let app = App::new().configure(views::views_factory);
		app
	})
		.bind(&server_address);
	
	match server_result {
		Ok(server_ok) => {
			// Print the server address before starting the server
			println!(
				"==========================\n\
				| http://{}  |\n\
				==========================",
				server_address
			);
			
			// Start the server and run it until it's shut down
			server_ok.run().await
		},
		Err(err) => {
			eprintln!("Failed to bind server: {}", err);
			Err(
				io::Error::new(io::ErrorKind::AddrNotAvailable,
					format!(
						"Failed to bind server to the specified address {}",
						server_address
					)
				))
		}
	}
}
//? ********************************************************
