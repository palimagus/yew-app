use yew::prelude::*;
use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or(AttrValue::from("Click"))]
  pub label: AttrValue,
  #[prop_or(AttrValue::from("Me daddy!"))]
  pub message: AttrValue,
}

#[function_component]
pub fn LogButton(props: &Props) -> Html {

  let message = props.message.clone();
  let onclick = Callback::from(move |_| {
    let log = format!("[LOG] {}", message);
    console::log_1(&log.into());
  });

  let log_button_classes = vec!["rounded-md px-2 py-1 bg-gray-200 hover:bg-gray-100"];

  html! {
    <button {onclick} class={classes!(log_button_classes)} >{props.label.clone()}</button>
  }
}