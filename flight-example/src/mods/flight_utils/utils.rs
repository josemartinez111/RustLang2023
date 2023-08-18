// FILE: mods/flight_utils/utils.rs
// ____________________________________________________

use serde_json::json;

use crate::mods::models::flight::Flight;
use crate::mods::utils::utilities::Unit;

// ____________________________________________________

fn airport_codes() -> Vec<String> {
  vec![
    String::from("JFK"),
    String::from("LAX"),
    String::from("ORD"),
    String::from("ATL"),
    String::from("DFW"),
    String::from("SFO"),
    String::from("MIA"),
    String::from("DEN"),
    String::from("SEA"),
    String::from("BOS"),
    String::from("IAD"),
  ]
}
// ____________________________________________________


pub fn print_flight_details(flight: &Flight, original_destination: &str) -> Unit {
  let ap_codes = airport_codes();
  
  let flight_info = json!({
  "Information for airport code": ap_codes[0],
  "Changing destination from": format!("{} to {}", original_destination, flight.destination),
  "Flight code": flight.code,
  "Flight destination": flight.destination,
  "Passengers": flight.passengers,
});
  
  println!("Flight-Info: {}", flight_info);
}
// ____________________________________________________