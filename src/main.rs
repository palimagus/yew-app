mod views;
mod components;

fn main() {
    yew::Renderer::<views::app::App>::new().render();
}
