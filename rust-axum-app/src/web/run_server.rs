// FILE: server/run_server.rs
// ___________________________________________________________

use std::net::SocketAddr;

use axum::Router;

// ___________________________________________________________

pub async fn run_server(routes: Router, addr: &SocketAddr) {
  println!("->> LISTENING on: http://{}/", addr);  // Make this clickable
  
  axum::Server::bind(&addr)
    .serve(routes.into_make_service())
    .await.unwrap_or_else(|err| {
      eprintln!("Server error: {}", err);
    });
}
// ___________________________________________________________