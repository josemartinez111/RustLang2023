// FILE: utils/error.rs
// ___________________________________________________________

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
// ___________________________________________________________

pub type CustomResult<CustomType> = Result<CustomType, Error>;
// ___________________________________________________________

#[derive(Debug)]
pub enum Error {
  LoginFail
}
// ___________________________________________________________

#[allow(unused_doc_comments)]
/// In Swift, you might use an extension to add methods or computed
/// properties to an existing type.
/// In Rust, the `impl` block serves a similar purpose;
/// it allows you to add methods to a type.
///
/// The `IntoResponse` is a trait, which is somewhat similar to a
/// protocol in Swift. When you write
/// `impl IntoResponse for Error`, you're saying that you're providing
/// an implementation of the `IntoResponse` trait for your custom `Error` type.
///
/// The `fn into_response(self) -> Response` line within the `impl`
/// block is like adding a method to conform to a protocol in Swift. In
/// this case, the `into_response` method takes ownership of `self`
/// (the `Error` instance) and returns a `Response`.
impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> {:>12} - {self:?}", "INTO_RES");
    let resp_body = "UNHANDLED_CLIENT_ERROR";
    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
    
    // Returning a tuple
    (status_code, resp_body).into_response()
  }
}
// ___________________________________________________________




