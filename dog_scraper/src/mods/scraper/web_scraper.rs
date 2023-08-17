// ! scraper/web_scraper.rs
//? ********************************************************

use reqwest::Error;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use crate::mods::site::criteria::ECriteria;
use crate::mods::site::site::Site;
//? ********************************************************

pub const DOG_BREED: &str = "Belgian Malinois";
pub const LOCATION: &str = "Saint Augustine";
pub const BING_API_KEY: &str = "38c517e3-2e19-4189-8fa7-219d4786bc19";
//? ********************************************************

#[derive(Debug, Clone, PartialEq)]
pub enum CriteriaType {
	PositiveReviews,
	VisitorCount,
	RedFlags,
	Distance,
}

impl CriteriaType {
	pub fn css_selector(&self) -> &str {
		match self {
			CriteriaType::PositiveReviews => ".reviews",
			CriteriaType::VisitorCount => ".visitors",
			CriteriaType::RedFlags => ".red-flags",
			CriteriaType::Distance => ".distance",
		}
	}
}
//? ********************************************************

// Extract the necessary data from the website at the given URL
pub async fn extract_data(url: &str) -> Result<Vec<ECriteria>, Error> {
	let resp = reqwest::get(url).await?;
	let body = resp.text().await?;
	let fragment = Html::parse_document(&body);
	
	let criteria_types = [
		CriteriaType::PositiveReviews,
		CriteriaType::VisitorCount,
		CriteriaType::RedFlags,
		CriteriaType::Distance,
	];
	
	let mut extracted_criteria = Vec::new();
	
	for criteria_type in &criteria_types {
		let selector = Selector::parse(criteria_type.css_selector()).unwrap();
		if let Some(element) = fragment.select(&selector).next() {
			let value = element.inner_html();
			match criteria_type {
				CriteriaType::PositiveReviews => {
					extracted_criteria.push(ECriteria::PositiveReviews(value.parse().unwrap_or(0)))
				}
				CriteriaType::VisitorCount => {
					extracted_criteria.push(ECriteria::VisitorCount(value.parse().unwrap_or(0)))
				}
				CriteriaType::RedFlags => {
					extracted_criteria.push(ECriteria::RedFlags(value.parse().unwrap_or(0)))
				}
				CriteriaType::Distance => {
					extracted_criteria.push(ECriteria::Distance(value.parse().unwrap_or(0.0 as u32)))
				}
			}
		}
	}
	
	Ok(extracted_criteria)
}

// Search for websites matching the given dog breed and location, and return the top 15 sites
pub async fn search_sites() -> Result<String, Error> {
	let search_query = format!("{} in {}", DOG_BREED, LOCATION);
	
	let client = reqwest::Client::new();
	let url = format!("https://api.bing.microsoft.com/v7.0/search?q={}&count=50", search_query);
	let mut headers = HeaderMap::new();
	headers.insert("Ocp-Apim-Subscription-Key", HeaderValue::from_str(BING_API_KEY).unwrap());
	headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
	
	let response = client.get(&url).headers(headers).send().await?;
	let data: Value = response.json().await?;
	
	let default_vec = Vec::new();
	let web_pages = data["webPages"]["value"].as_array().unwrap_or(&default_vec);
	let mut sites = Vec::new();
	
	for web_page in web_pages {
		if let Some(url) = web_page["url"].as_str() {
			match extract_data(url).await {
				Ok(criteria) => {
					let site = Site {
						url: url.to_string(),
						criteria,
					};
					sites.push(site);
				}
				Err(e) => eprintln!("Error extracting data from {}: {}", url, e),
			}
		}
	}
	
	// Sorting and filtering logic based on the criteria
	sites.sort_by(|a, b| {
		let a_score = a.criteria_score();
		let b_score = b.criteria_score();
		b_score.partial_cmp(&a_score).unwrap()
	});
	
	let top_sites = sites.into_iter().take(15).collect::<Vec<_>>();
	
	let results = json!({
            "results": top_sites
        });
	
	Ok(results.to_string())
}
//? ********************************************************
