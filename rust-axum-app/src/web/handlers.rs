// FILE: web/health_check
// ___________________________________________________________

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};

use crate::models::health::HealthParams;
// ___________________________________________________________

pub async fn health_check(Query(params): Query<HealthParams>) -> impl IntoResponse {
print!("->> {:<12} - health_check - {params:?}", "HANDLER");
  let name = params.name.as_deref().unwrap_or("MESSED UP...");
  Html(format!("Running a <strong>{name}</strong>"))
}


pub async fn health_check2(Path(name): Path<String>) -> impl IntoResponse {
print!("->> {:<12} - health_check2 - {name:?}", "HANDLER");
  Html(format!("Running a <strong>{name}</strong>"))
}
// ___________________________________________________________