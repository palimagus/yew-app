use yew::prelude::*;

use crate::components::{app_navbar::AppNavbar, portfolio_card::PortfolioCard};

pub struct Portfolio;

impl Component for Portfolio {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    let generated_portfolios: Vec<Html> = (0..10)
      .map(|_| {
        html! {
          <PortfolioCard />
        }
      })
      .collect();

    html! {
      <div class={"app"}>
        <AppNavbar />
        <div class={"portfolio"}>
          {generated_portfolios}
        </div>
      </div>
    }
  }
}