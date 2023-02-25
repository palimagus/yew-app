use yew::prelude::*;

pub struct PortfolioCard;

impl Component for PortfolioCard {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class={"portfolio-card"}></div>
    }
  }
}

