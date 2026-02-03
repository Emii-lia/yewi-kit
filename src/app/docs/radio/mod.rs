use yew::{function_component, html, Html};
use crate::previews::RadioPreview;

#[function_component(RadioPage)]
pub(crate) fn radio_page() -> Html {
  html! {
    <div class="RadioPage page-container">
      <RadioPreview/>
    </div>
  }
}