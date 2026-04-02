use yew::{component, html, Html};
use crate::features::InstallationSection;

#[component(InstallationPage)]
pub(crate) fn installation_page() -> Html {
  html! {
    <div class="InstallationPage page-container">
      <InstallationSection/>
    </div>
  }
}