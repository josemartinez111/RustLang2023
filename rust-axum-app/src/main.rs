// main.rs
// ________________________________________________________

use std::net::SocketAddr;

use axum::Router;

use rust_axum_app::{
  utils::utilities::format_print,
  web::router::{init_routes, routes_static},
  web::run_server::run_server,
  web::auth
};

// ________________________________________________________

// __________________________________________________
#[tokio::main]
async fn main() {
  format_print("_", 45);
  // __________________________________________________
  
  // Routes
  let all_routes = Router::new()
    .merge(init_routes())
    .merge(auth::routes_login::login_routes())
    .fallback_service(routes_static());
  
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