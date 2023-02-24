use yew::prelude::*;

// enum Links {
//   Portfolio,
//   Blog,
//   About,
//   Contact,
// }

// enum MiniLinks {
//   Github,
//   CV,
// }

#[function_component(AppNavbar)]
pub fn navbar() -> Html {
  let navbar_classes = vec!["w-full", "h-32", "flex", "justify-between", "items-center"];
  let side_div_classes = vec!["w-full", "h-full", "flex", "justify-evenly", "items-center"];
  html! {
    <nav class={classes!(navbar_classes)}>
      <div class={classes!(side_div_classes.clone())}>
        <a href={"#"}>{ "Portfolio" }</a>
        <a href={"#"}>{ "Blog" }</a>
      </div>
      <div class={classes!("h-full")}>
        <img src={"assets/logo.png"} class={classes!("logo")} alt={"logo"} />
      </div>
      <div class={classes!(side_div_classes.clone())}>
        <a href={"#"}>{ "About" }</a>
        <a href={"#"}>{ "Contact" }</a>
      </div>
    </nav>
  }
}