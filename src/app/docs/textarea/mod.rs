use yew::{component, html, Html};
use crate::previews::TextareaPreview;

#[component(TextareaPage)]
pub(crate) fn textarea_page() -> Html {
  html! {
    <div class="TextareaPage page-container">
      <TextareaPreview/>
    </div>
  }
}