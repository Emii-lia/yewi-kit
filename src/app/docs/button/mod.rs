use yew::{component, html, Html};
use crate::previews::ButtonPreview;

#[component(ButtonPage)]
pub(crate) fn button_page() -> Html {
  html! {
    <div class="ButtonPage page-container">
      <ButtonPreview/>
    </div>
  }
}