// FILE: web/auth/routes_login.rs
// ___________________________________________________________

use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json, Value};

use crate::{
  models::login::LoginPayload,
  utils::error::{CustomResult, Error}
};
// ___________________________________________________________

pub fn login_routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}
// ___________________________________________________________

async fn api_login(payload: Json<LoginPayload>) -> CustomResult<Json<Value>>  {
  println!("->> {:<12} - api_login", "HANDLER");
  
  if payload.username != "demo1" || payload.pwd != "welcome" {
    return Err(Error::LoginFail);
  }
  
  // TODO: Set cookies
  
  // Create the success body
  let body = Json(json!({
    "result": {
      "success": true,
    }
  }));
  
  Ok(body)
}
// ___________________________________________________________