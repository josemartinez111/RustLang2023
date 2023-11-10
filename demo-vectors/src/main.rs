// main.rs
// ________________________________________________________
// imports from library of the whole project
use demo_vectors::{
  models::coffee::Coffee,
  utils::utils::{
    format_print,
    print_coffee_list
  }
};

// ________________________________________________________

fn main() {
  format_print("_", 45);
  // __________________________________________________
  
  // Use `Vec::new()` for an empty vector, or `vec![]` for initial elements.
  // Examples: `Vec::new()` or `vec![Coffee::default(); 10]`
  #[allow(unused_mut, unused_variables)]
    let mut coffees_data: Vec<Coffee> = Vec::new();
  
  coffees_data.extend(vec![
    Coffee::new(1, "Espresso"),
    Coffee::new(2, "Cappuccino"),
    Coffee::new(3, "Latte"),
    Coffee::new(4, "Mocha"),
  ]);
  
  // coffees_data.iter().enumerate().for_each(|(index, &coffee)| {
  //   // Use the index directly for id, which starts at 0
  //   let new_coffee = Coffee::new(index as i32, coffee.name.as_str());
  //   coffees_data.push(new_coffee);
  // });
  
  
  // Print each Coffee instance in JSON format.
  print_coffee_list("Coffee", &coffees_data);
  println!();
  
  // Create a vector with some dummy coffees
  let coffees_data2 = vec![
    Coffee::new(1, "Doppio"),
    Coffee::new(2, "Cortado"),
    Coffee::new(3, "Ristretto"),
    Coffee::new(4, "Americano"),
  ];
  
  // Print each Coffee instance in JSON format.
  print_coffee_list("Coffee", &coffees_data2);
  println!("-----------------------------------\n");
  
  // Print initial list
  println!("Initial list of coffees:\n");
  print_coffee_list("Coffee", &coffees_data); // Use a reference to avoid moving coffees_data
  println!("\n***********************************\n");
  
  // Remove by Index
  println!("Removing element at index 1:\n");
  coffees_data.remove(1); // Removes the element at index 1
  print_coffee_list("Coffee", &coffees_data);
  println!("***********************************\n");
  
  let pushed_new_coffee = coffees_data.iter()
    .map(|coffee: &Coffee| coffee.id)
    .max()
    .unwrap_or(0) + 1;
  
  // `<insert>`: Inserts an element at position index within
  // the vector, shifting all elements after it to the right
  coffees_data.insert(
    0, Coffee::new(
    pushed_new_coffee,
    "Moca-Chino"
  ));
  
  // Correct the IDs to maintain order
  // `<iter_mut>`: Returns an iterator that allows modifying each
  // value. The iterator yields all items from start to end.
  coffees_data.iter_mut()
    .enumerate()
    .for_each(|(index, coffee)| coffee.id = index as i32);
  
  // Pop an Element from the End
  println!("Popping the last element:\n");
  coffees_data.pop(); // Removes the last element
  print_coffee_list("Coffee", &coffees_data);
  println!("***********************************\n");
  
  // Remove "Cappuccino" coffees using the retain function
  println!("Retaining coffees that are not 'Cappuccino':\n");
  // Retains only the elements specified by the predicate.
  coffees_data.retain(|coffee: &Coffee| coffee.name != "Cappuccino");
  print_coffee_list("Coffee", &coffees_data);
  println!("***********************************\n");
  
  // Retain only coffees that do not contain "Latte" in their name
  println!("Retaining coffees that do not contain 'Espresso':");
  
  // `<into_iter>`: Creates a consuming iterator, that is, one
  // that moves each value out of the vector (from start to end).
  // The vector cannot be used after calling this.
  let filtered_coffees: Vec<Coffee> = coffees_data.into_iter()
    .filter(|coffee: &Coffee| !coffee.name.contains("Latte"))
    .collect();
  
  // Reassign the filtered vector back to coffees_data
  coffees_data = filtered_coffees;
  print_coffee_list("Coffee", &coffees_data);
  // __________________________________________________
  format_print("_", 45);
}
// __________________________________________________________










