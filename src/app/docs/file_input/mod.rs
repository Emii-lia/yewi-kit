use yew::{component, html, Html};
use crate::previews::FileInputPreviews;

#[component(FileInputPage)]
pub(crate) fn file_input_page() -> Html {
  html! {
    <div class="FileInputPage page-container">
      <FileInputPreviews/>
    </div>
  }
}