use yew::{component, html, Html};
use crate::previews::RadioPreview;

#[component(RadioPage)]
pub(crate) fn radio_page() -> Html {
  html! {
    <div class="RadioPage page-container">
      <RadioPreview/>
    </div>
  }
}