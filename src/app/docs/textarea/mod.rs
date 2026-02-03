use yew::{function_component, html, Html};
use crate::previews::TextareaPreview;

#[function_component(TextareaPage)]
pub(crate) fn textarea_page() -> Html {
  html! {
    <div class="TextareaPage page-container">
      <TextareaPreview/>
    </div>
  }
}