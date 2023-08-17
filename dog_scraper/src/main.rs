// This code should be placed in src/main.rs
mod mods;
//? ********************************************************

use std::error::Error;
use reqwest::header::LOCATION;
use crate::mods::scraper::web_scraper::{DOG_BREED, search_sites};
//? ********************************************************

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("Searching for websites with information about {} in {}...", DOG_BREED, LOCATION);
	
	match search_sites().await {
		Ok(result) => {
			println!("Top 15 sites based on the specified criteria:");
			println!("{}", result);
		}
		Err(e) => {
			eprintln!("Error: {}", e);
		}
	}
	
	Ok(())
}
//? ********************************************************
