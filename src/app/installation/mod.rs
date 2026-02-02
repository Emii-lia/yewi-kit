use yew::{function_component, html, Html};
use crate::features::InstallationSection;

#[function_component(InstallationPage)]
pub fn installation_page() -> Html {
  html! {
    <div class="InstallationPage page-container">
      <InstallationSection/>
    </div>
  }
}