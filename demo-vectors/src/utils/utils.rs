// FILE: utils/utils.rs
// ____________________________________________________

use crate::models::coffee::Coffee;

pub type Void = ();
// ____________________________________________________

pub fn format_print(arg: &str, num: usize) -> Void {
  println!("{}\n", arg.repeat(num));
}
// ____________________________________________________

pub fn print_coffee_list(obj_str: &str, coffees_data2: &Vec<Coffee>) -> Void {
  coffees_data2.iter().for_each(|coffee: &Coffee| {
    println!("{}: {}", obj_str.to_owned(),coffee.to_json());
  });
}
// ____________________________________________________