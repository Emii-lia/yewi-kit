use yew::{component, html, Html};
use crate::features::Documentation;

#[component(DocumentationPage)]
pub fn documentation_page() -> Html {
  html! {
    <div class="DocumentationPage page-container">
      <Documentation/>
    </div>
  }
}