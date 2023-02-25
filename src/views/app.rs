use yew::prelude::*;

use crate::components::{app_navbar::AppNavbar, basic_container::BasicContainer};

pub struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class={"app"}>
        <AppNavbar />
        <div class={"wrapper"}>
          <BasicContainer>
            <h1 class={"intro-title"}>
              {"Palimagus"}
            </h1>
            <h2 class={"intro-sub"}>
              {"When the web is magic! ✨"}
            </h2>
            <p class={"intro-desc"}>
              {"Welcome to Palimagus, a community of developers and designers who are passionate about creating beautiful and functional web applications."}
            </p>
          </BasicContainer>
          <BasicContainer>
            <h2 class={"intro-sub"}>
              {"Who are we?"}
            </h2>
            <p class={"intro-desc"}>
              {"We offer a wide range of services, from web design to web development, and everything in between."}
            </p>
            <p class={"intro-desc"}>
              {"We work with technologies such as Wordpress, Javascript frameworks and web assembly."}
            </p>
          </BasicContainer>
        </div>
      </div>
    }
  }

}