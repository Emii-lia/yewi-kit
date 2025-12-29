use yew::{function_component, html, Html};
use crate::previews::CollapsePreview;

#[function_component(CollapsePage)]
pub(crate) fn collapse_page() -> Html {
  html! {
    <div class="CollapsePage page-container">
      <CollapsePreview/>
    </div>
  }
}