// main.rs
use crate::mods::models::object::*;
use crate::mods::utils::utilities::format_print;

mod mods;

fn main() {
    format_print("=");
    // ======================================================
    let obj1 = Object {
        width: 35,
        height: 55,
    };

    let obj2 = Object::new(57, 83);

    println!("*. {}x{} with area: {}", obj1.width, obj1.height, obj1.area());
    println!("*. {}x{} with area: {}", obj2.width, obj2.height, obj2.area());

    println!("*. obj1:\n{:#?}", obj1);
    println!("*. obj2:\n{:#?}", obj2);

    // using display
    println!("*. obj1 impl Display:\n{}", obj1);
    println!("*. obj2 impl Display:\n{}", obj2);

    // ======================================================
    format_print("=");
}


