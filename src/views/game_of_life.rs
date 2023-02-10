use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{function_component, html, Html};
use yew_canvas::{Canvas, WithRander};

#[derive(Clone, PartialEq)]
struct Rander();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let interface: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        interface.set_fill_style(&"red".into());
        interface.fill_rect(0.0, 0.0, 100.0, 100.0);
    }
}

#[function_component(GameOfLife)]
pub fn game_of_life() -> Html {
    html! {
        <Canvas<CanvasRenderingContext2d, Rander>
            //Just use style, canvas can suit automaticly.
            style="
                width: 2000px;
                height: 200px;
            "
            rander={Box::new(Rander())}
        >
            {"The browser is not supported."}
        </Canvas<CanvasRenderingContext2d, Rander>>
    }
}
