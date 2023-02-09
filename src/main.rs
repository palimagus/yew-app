use yew::prelude::*;

use crate::components::log_button::LogButton;
use crate::components::basic_container::BasicContainer;

mod components {
    pub mod log_button;
    pub mod basic_container;
}

#[function_component(App)]
fn app() -> Html {
    let danger_message_classes = vec!["bg-red-100", "text-red-500", "p-2", "rounded-md"];
    html! {
        <BasicContainer>
            <h1>{ "The name ? Is Anorak !" }</h1>
            <p class={classes!(danger_message_classes)}>
                { "Watchout! It's a wasm!" }
            </p>
            <LogButton />
        </BasicContainer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
