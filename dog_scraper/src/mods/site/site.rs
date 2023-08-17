// site/site.rs
//? ********************************************************

use serde::{Deserialize, Serialize};

use crate::mods::site::criteria::ECriteria;

//? ********************************************************


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Site {
  pub url: String,
  pub criteria: Vec<ECriteria>,
}


impl Site {
  pub fn criteria_score(&self) -> f32 {
    self.criteria.iter().map(|criterion: &ECriteria| {
      match criterion {
        ECriteria::PositiveReviews(value) => *value as f32,
        ECriteria::VisitorCount(value) => *value as f32,
        ECriteria::RedFlags(value) => -(*value as f32),
        ECriteria::Distance(value) => -(*value as f32),
      }
    }).sum()
  }
}


//? ********************************************************