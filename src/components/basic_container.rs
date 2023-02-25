use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component]
pub fn BasicContainer(props: &Props) -> Html {
  html! {
    <div class={"basic-container"}>
      {for props.children.iter()}
    </div>
  }
}