use yew::{component, html, Html};
use crate::previews::CheckboxPreview;

#[component(CheckboxPage)]
pub fn checkbox_page() -> Html {
  html! {
    <div class="CheckboxPage page-container">
      <CheckboxPreview/>
    </div>
  }
}