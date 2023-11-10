// FILE: models/coffee.rs
// ___________________________________________________________

// use std::fmt::Display;

use serde::{Deserialize, Serialize};

// ___________________________________________________________


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Coffee {
  pub id: i32,
  pub  name: String,
}
// ___________________________________________________________

impl Coffee {
  pub fn new(id: i32, name: &str) -> Self {
    // to_owned(): Creates owned data from
    // borrowed data, usually by cloning
    Self { id, name: name.to_owned() }
  }
  
  // Serialize the Coffee instance to a JSON string, handling errors
  pub fn to_json(&self) -> String {
    serde_json::to_string_pretty(self).unwrap_or_else(
      |err: serde_json::Error| {
        format!("Failed to serialize coffee: {}", err)
      })
  }
}

// impl Display for Coffee {
//   fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//     write!(f, "id: {}, name: {}", self.id, self.name)
//   }
// }
// ___________________________________________________________