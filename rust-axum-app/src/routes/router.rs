// FILE: routes/router.rs
// ___________________________________________________________

use axum::Router;
use axum::routing::{get, get_service};
use tower_http::services::ServeDir;

use crate::routes::handlers::{health_check, health_check2};

// ___________________________________________________________

pub fn init_routes() -> Router {
  Router::new()
    .route("/health-check", get(health_check))
    .route("/health-check2/:name", get(health_check2))
}

pub fn routes_static() -> Router {
  Router::new()
    .nest_service("/", get_service(ServeDir::new("./")))
}
// ___________________________________________________________