// main.rs
// ________________________________________________________

use crate::mods::flight_utils::utils::print_flight_details;
use crate::mods::models::flight::Flight;
use crate::mods::utils::utilities::format_print;

mod mods;
// ________________________________________________________

// __________________________________________________

fn main() {
  format_print("_");
  // __________________________________________________
  
  
  let passengers = vec![String::from("John Doe"), String::from("Jane Smith")];
  let mut flight = Flight::new("AA123", "LAX", passengers);
  
  print_flight_details(&flight, "JFK"); // Assuming JFK is the original destination
  flight.change_destination("ORD");
  print_flight_details(&flight, "LAX"); // LAX was the original destination before the change
  // __________________________________________________
  format_print("_");
}
// __________________________________________________________