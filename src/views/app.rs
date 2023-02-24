use yew::prelude::*;

use crate::components::basic_container::BasicContainer;
use crate::components::log_button::LogButton;
use crate::components::app_navbar::AppNavbar;

pub struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    let danger_message_classes = vec!["bg-red-100", "text-red-500", "p-2", "rounded-md"];
    html! {
      <>
        <AppNavbar />
        <BasicContainer>
          <h1>{ "The name ? Is Anorak !" }</h1>
          <p class={classes!(danger_message_classes)}>
            { "Watchout! It's a wasm!" }
          </p>
          <LogButton />
        </BasicContainer>
      </>
    }
  }

}