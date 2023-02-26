use yew::prelude::*;

pub struct PortfolioCard {
  pub bg_img_path: String, 
}

impl Component for PortfolioCard {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self {
      bg_img_path: "".to_string(),
    }
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class={"portfolio-card"}></div>
    }
  }
}

