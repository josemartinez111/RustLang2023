// !src/handlers.rs
use std::sync::MutexGuard;
use actix_web::{HttpResponse, web};
use crate::state::AppSate;
//? ********************************************************

pub async fn health_check_handler(app_state: web::Data<AppSate>) -> HttpResponse {
	let health_check_response: &String = &app_state.health_check_response;
	let mut visit_count: MutexGuard<u32> = app_state.visit_count.lock()
		.map_or_else(|poisoned| {
			// !map_or_else: This function can be used to unpack
			// !a successful result while handling an error.
			eprintln!("Visit count mutex is poisoned!");
			// !into_inner: consumes this error indicating that a lock is poisoned,
			// !returning the underlying guard to allow access regardless
			poisoned.into_inner()
		}, |guard| guard, );
	
	let response: String = format!(
		"{} {} times",
		health_check_response,
		visit_count
	);
	
	*visit_count += 1;
	let result = HttpResponse::Ok().json(&response);
	
	result
}
//? ********************************************************
//? ********************************************************