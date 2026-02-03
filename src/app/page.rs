use yew::{function_component, html, Html};
use crate::features::{Hero, HomeHeader};

#[function_component]
pub(crate) fn Home() -> Html {
  html! {
    <div class="Home">
      <HomeHeader/>
      <Hero/>
    </div>
  }
}