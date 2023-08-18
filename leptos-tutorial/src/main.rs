// main.rs
// ________________________________________________________

mod mods;
pub mod app;
pub mod global_styles;

use leptos::{mount_to_body, view};
use app::App;
// ________________________________________________________

fn main() {
  mount_to_body(|cx| view! { cx, <App/> });
}
// __________________________________________________________
