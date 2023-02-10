use yew::prelude::*;

use crate::components::basic_container::BasicContainer;
use crate::components::log_button::LogButton;

mod components {
    pub mod basic_container;
    pub mod log_button;
}

mod views {
    pub mod game_of_life;
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
