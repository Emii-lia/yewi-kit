use yew::{function_component, html, Html};
use crate::previews::CheckboxPreview;

#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
  html! {
    <div class="CheckboxPage page-container">
      <CheckboxPreview/>
    </div>
  }
}