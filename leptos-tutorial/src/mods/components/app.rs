// FILE: App
// ____________________________________________________

use leptos::{
  component,
  create_signal,
  ev::Event,
  event_target_value,
  IntoView,
  Scope,
  SignalGet,
  view,
};

use crate::mods::components::global_styles::*;
// ____________________________________________________

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  let (text, set_text) = create_signal(cx, "".to_string());
  let (input_value, set_input) = create_signal(cx, "".to_string());
  
  let is_text_empty = move || match text.get().is_empty() {
    true => H1_STYLE.to_string(),
    false => H1_STYLE.replace("opacity: 0;", "opacity: 1;"),
  };
  // ___________________ FUNCTIONS ___________________
  
  let handle_show_text = move |_| {
    set_text(String::from("@App component"));
  };
  
  let handle_hide_text = move |_| {
    set_text(String::from(""));
    set_input(String::from(""));
  };
  
  let handle_input_change = move |ev: Event| {
    set_input(event_target_value(&ev));
  };
  
  let handle_update_text = move |_| {
    set_text(input_value.get()); // Directly use set_text here
  };
  // ___________________ MARKUP-VIEW ___________________
  view! { cx,
    <div style=CONTAINER>
      /* show-button */
      <button
        style=BUTTON_STYLE
        on:click=handle_show_text
      >
        Show Text
      </button>
      /* hide-button */
      <button
        style=BUTTON_STYLE
        on:click=handle_hide_text
      >
        Hide Text
      </button>
      /* input field */
      <input
        type="text"
        style=INPUT_STYLE
        prop:value={input_value}
        on:input=handle_input_change
      />
      /* update-button */
      <button
        style=BUTTON_STYLE
        on:click=handle_update_text>
        Update Text
      </button>
      <h1 style=is_text_empty>{text}</h1>
    </div>
    }
}
// ____________________________________________________
