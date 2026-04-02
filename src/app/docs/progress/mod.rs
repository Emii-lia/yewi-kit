use yew::{component, html, Html};
use crate::previews::ProgressPreview;

#[component(ProgressPage)]
pub(crate) fn progress_page() -> Html {
  html! {
    <div class="ProgressPage page-container">
      <ProgressPreview/>
    </div>
  }
}