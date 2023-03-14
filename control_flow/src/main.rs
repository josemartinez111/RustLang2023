// main.rs

use crate::mods::utils::utilities::format_print;

mod mods;

// *********************************************************
// *********************************************************

fn main() {
    format_print("*", 45);
    // --------------------------------------------------

    let n = 6;

    if n % 4 == 0 {
        println!("*. {} is divisible by 4", n);
    } else if n % 3 == 0 {
        println!("*. {} is divisible by 3", n);
    } else if n % 2 == 0 {
        println!("*. {} is divisible by 2", n);
    } else {
        println!("*. {} is not divisible by 4, 3, or 2", n)
    }
    // --------------------------------------------------
    format_print("*", 45);
}
// *********************************************************