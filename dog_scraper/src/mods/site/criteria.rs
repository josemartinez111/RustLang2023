// ! site/criteria.rs
//? ********************************************************
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
//? ********************************************************

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ECriteria {
	PositiveReviews(u32),
	VisitorCount(u32),
	RedFlags(u32),
	Distance(u32),
}

impl Display for ECriteria {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			ECriteria::PositiveReviews(value) => write!(f, "Positive Reviews: {}", value),
			ECriteria::VisitorCount(value) => write!(f, "Visitor Count: {}", value),
			ECriteria::RedFlags(value) => write!(f, "Red Flags: {}", value),
			ECriteria::Distance(value) => write!(f, "Distance: {:.2} miles", value),
		}
	}
}
//? ********************************************************












