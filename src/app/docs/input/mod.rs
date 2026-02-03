use yew::{function_component, html, Html};
use crate::previews::InputPreview;

#[function_component(InputPage)]
pub(crate) fn input_page() -> Html {
  html! {
    <div class="InputPage page-container">
      <InputPreview/>
    </div>
  }
}