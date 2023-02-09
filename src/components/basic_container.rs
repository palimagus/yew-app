use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component]
pub fn BasicContainer(props: &Props) -> Html {
  let container_classes = vec!["p-4", "m-2", "border-[1px]", "border-gray-400", "flex", "flex-col", "gap-2"];
  html! {
    <div class={classes!(container_classes)}>
      {for props.children.iter()}
    </div>
  }
}