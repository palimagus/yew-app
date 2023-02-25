use yew::prelude::*;

#[function_component(AppNavbar)]
pub fn navbar() -> Html {
  html! {
    <nav class={"navbar"}>
      <div class={"divs"}>
        <a href={"/portfolio"}>{ "portfolio" }</a>
        <a href={"#"}>{ "blog" }</a>
      </div>
      <a href={"/"} class={"h-full"}>
        <img src={"assets/logo.png"} class={"logo"} alt={"logo"} />
      </a>
      <div class={"divs"}>
        <a href={"#"}>{ "about" }</a>
        <a href={"#"}>{ "contact" }</a>
      </div>
    </nav>
  }
}