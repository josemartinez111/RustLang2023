// main.rs
// ________________________________________________________

use std::collections::{HashMap, BTreeMap};

use demo_hashmaps::{
  models::coffee::Coffee,
  utils::utils::format_print,
};

// ________________________________________________________

// __________________________________________________

fn main() {
  format_print("_", 45);
  // __________________________________________________
  
  #[allow(unused_mut)]
  let mut coffee_map: HashMap<&str, Coffee> = HashMap::from([
    ("Arabica", Coffee::new(1000, 10)),
    ("Robusta", Coffee::new(2000, 40)),
    ("Liberica", Coffee::new(3000, 50)),
    ("Excelsa", Coffee::new(4000, 500)),
  ]);
  
  // Using for_each to iterate over coffee_map
  // and print each Coffee object as a JSON string
  coffee_map.iter().for_each(|(&name, coffee)| {
    println!("{}: {}", name, coffee.to_json());
  });
  println!("\n=============================================");
  
  print!("get: Returns a Coffee object from the coffee_map\n\n");
  println!("*. Coffee (Robusta): {:?}", coffee_map.get("Robusta"));
  
  print!("\n\nentry: if the entry does not exist, it will be created\n\n");
  coffee_map.entry("Moca-Chino")
    .or_insert(
      Coffee::new(5000, 1)
    );
  
  println!("*. Coffee (Moca-Chino): {:?}", coffee_map.get("Moca-Chino"));
  println!("\n=============================================");
  
  // BTreeMap: sorts by count
  #[allow(unused_mut)]
  let mut coffee_btree_map: BTreeMap<Coffee, &str> = BTreeMap::from([
    (Coffee::new(3000, 1), "Medium Fruity"),
    (Coffee::new(2000, 2), "Ethiopian Blend"),
    (Coffee::new(1000, 5), "Bold, Rich Flavor"),
  ]);
  
  // Using for_each to iterate over coffee_btree_map
  // and print each Coffee object as a JSON string
  coffee_btree_map.iter().for_each(|(coffee, &name)| {
    println!("{}: {}", name, coffee.to_json());
  });
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________