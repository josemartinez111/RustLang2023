// !tutor-nodb/src/bin/tutor-service.rs
#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../types.rs"]
mod types;

use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use actix_web::web::{Data};
use crate::routes::*;
use crate::state::AppSate;
use crate::types::Void;
//? ********************************************************

#[actix_rt::main]
async fn main() -> io::Result<Void> {
	// *Set the host address for the server to listen on
	let host_address: &str = "127.0.0.1:3000";
	
	// *Create an AppSate object that holds the initial state for the application
	let app_state = AppSate {
		health_check_response: "I'm good. You've already asked me ".to_string(),
		visit_count: Mutex::new(0),
	};
	
	// *Wrap the AppSate object in a web::Data container
	// *and store it as shared_data for the web application to use
	let shared_data: Data<AppSate> = web::Data::new(app_state);
	
	// *Define the web application using the shared data object and routing rules
	let app = move || {
		App::new()
			.app_data(shared_data.clone())
			.configure(general_routes)
	};
	
	// *Bind the web application to the host address and start the server
	let server = HttpServer::new(app)
		.bind(host_address)?;
	
	// *Check if the server is listening on any addresses, and get the first address
	match server.addrs().first() {
		Some(address) => {
			let port: u16 = address.port();
			// !Displaying server address in the terminal
			println!("Server is running on http://{}:{}", address.ip(), port);
		}
		None => {
			eprintln!("No listening addresses found");
		}
	}
	
	// *Start the server and return a success value if it runs successfully
	server.run().await?;
	Ok(())
}
//? ********************************************************

