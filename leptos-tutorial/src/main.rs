// main.rs
// ________________________________________________________

mod mods;

use leptos::{mount_to_body, view};
use crate::mods::components::app::App;
// ________________________________________________________

fn main() {
  mount_to_body(|cx| view! { cx, <App/> });
}
// __________________________________________________________