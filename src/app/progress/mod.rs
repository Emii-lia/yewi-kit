use yew::{function_component, html, Html};
use crate::previews::ProgressPreview;

#[function_component(ProgressPage)]
pub(crate) fn progress_page() -> Html {
  html! {
    <div class="ProgressPage page-container">
      <ProgressPreview/>
    </div>
  }
}