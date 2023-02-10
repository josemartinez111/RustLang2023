// main.rs
use crate::mods::utils::utilities::calculate_weight_on_mars;

mod mods;

fn main() {
    println!("{}\n", "=".repeat(35));
    // ====================================================


    println!("*. Weight: {}", calculate_weight_on_mars(20.1));
    // ====================================================
    println!("{}", "=".repeat(35));
}

