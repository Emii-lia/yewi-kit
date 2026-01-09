use yew::{function_component, html, Html};
use crate::features::InstallationSection;

#[function_component]
pub(crate) fn Home() -> Html {
  html! {
    <div class="Home page-container">
      <InstallationSection/>
    </div>
  }
}