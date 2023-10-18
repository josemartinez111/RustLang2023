// main.rs
// ________________________________________________________

use std::net::SocketAddr;
use axum::Router;

use crate::routes::router::init_routes;
use crate::server::run_server::run_server;
use crate::utils::utilities::format_print;

mod routes { pub mod handlers; pub mod router; }
mod server { pub mod run_server; }
mod utils { pub mod utilities; }
mod models { pub mod health; }
// ________________________________________________________

// __________________________________________________
#[tokio::main]
async fn main() {
  format_print("_", 45);
  // __________________________________________________
  
  // Routes
  let all_routes = Router::new().merge(init_routes());
  
  // Server Address
  let url = ([127, 0, 0, 1], 8080);
  let addr = SocketAddr::from(url);
  
  // Start Server
  run_server(all_routes, &addr)
    .await;
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________