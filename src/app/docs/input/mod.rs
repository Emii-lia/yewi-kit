use yew::{component, html, Html};
use crate::previews::InputPreview;

#[component(InputPage)]
pub(crate) fn input_page() -> Html {
  html! {
    <div class="InputPage page-container">
      <InputPreview/>
    </div>
  }
}