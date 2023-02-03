use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let danger_message_classes = vec!["bg-red-100", "text-red-500", "p-2", "rounded-md"];
    html! {
        <div class={classes!("p-4")}>
            <h1>{ "The name ? Is Anorak !" }</h1>
            <p class={classes!(danger_message_classes)}>
                { "Watchout! It's a wasm!" }
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
