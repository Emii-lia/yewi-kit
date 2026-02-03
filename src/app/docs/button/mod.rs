use yew::{function_component, html, Html};
use crate::previews::ButtonPreview;

#[function_component(ButtonPage)]
pub(crate) fn button_page() -> Html {
  html! {
    <div class="ButtonPage page-container">
      <ButtonPreview/>
    </div>
  }
}