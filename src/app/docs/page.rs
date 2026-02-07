use yew::{function_component, html, Html};
use crate::features::Documentation;

#[function_component(DocumentationPage)]
pub fn documentation_page() -> Html {
  html! {
    <div class="DocumentationPage page-container">
      <Documentation/>
    </div>
  }
}