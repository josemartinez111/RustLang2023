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

// ____________________________________________________

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  let (text, set_text) = create_signal(cx, "".to_string());
  let (input_value, set_input) = create_signal(cx, "".to_string());
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
    <div>
      /* show-button */
      <button on:click=handle_show_text>
        Show Text
      </button>
      /* hide-button */
      <button on:click=handle_hide_text>
        Hide Text
      </button>
      /* input field */
      <input
        type="text"
        prop:value={input_value}
        on:input=handle_input_change
      />
      /* update-button */
      <button on:click=handle_update_text>
        Update Text
      </button>
      <h1>{text}</h1>
    </div>
    }
}
// ____________________________________________________
