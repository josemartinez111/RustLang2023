// FILE: mods/models/flight.rs
// ____________________________________________________

use serde::Serialize;
// ____________________________________________________

#[derive(Serialize)]
pub struct Flight {
  pub code: String,
  pub destination: String,
  pub passengers: Vec<String>,
}
// ____________________________________________________

impl Flight {
  pub(crate) fn new(code: &str, destination: &str, passengers: Vec<String>) -> Self {
    Self {
      code: String::from(code),
      destination: String::from(destination),
      passengers: passengers.iter().map(|passenger| {
        String::from(passenger)
      }).collect(),
    }
  }
  
  pub(crate) fn change_destination(&mut self, new_destination: &str) {
    self.destination = String::from(new_destination);
  }
}
// ____________________________________________________