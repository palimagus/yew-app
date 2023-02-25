use yew::prelude::*;
use yew_router::prelude::*;

mod views;
mod components;

#[derive(Clone, Routable, PartialEq)]
enum Routes {
  #[at("/")]
  Home,
  #[at("/portfolio")]
  Portfolio,
  #[at("/contact")]
  Contact,
  #[at("/about")]
  About,
  #[at("/blog")]
  Blog,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn switch(routes: Routes) -> Html {
  match routes {
    Routes::Portfolio => html! { <crate::views::portfolio::Portfolio /> },
    _ => html! { <crate::views::app::App /> },
  }
}

#[function_component(Main)]
fn main_app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
