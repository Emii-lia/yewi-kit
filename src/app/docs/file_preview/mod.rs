use yew::{component, html, Html};
use crate::previews::file_preview_preview::FilePreviewPreview;

#[component(FilePreviewPage)]
pub fn file_preview_page() -> Html {
  html! {
    <div class="FilePreviewPage page-container">
      <FilePreviewPreview/>
    </div>
  }
}