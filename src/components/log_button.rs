use yew::prelude::*;
use yew_icons::{Icon, IconId};

use gloo_console::log;

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
        log!(log);
    });

    let log_button_classes =
        vec!["flex gap-2 items-center rounded-md px-2 py-1 bg-gray-200 hover:bg-gray-100"];

    html! {
      <button {onclick} class={classes!(log_button_classes)} >
        <Icon icon_id={IconId::LucideHome} width={"16px".to_owned()} height={"16px".to_owned()} />
        {props.label.clone()}
      </button>
    }
}
