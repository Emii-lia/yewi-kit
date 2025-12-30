use yew::{function_component, html, Html};
use crate::previews::FileInputPreviews;

#[function_component(FileInputPage)]
pub(crate) fn file_input_page() -> Html {
  html! {
    <div class="FileInputPage page-container">
      <FileInputPreviews/>
    </div>
  }
}