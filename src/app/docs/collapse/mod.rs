use yew::{component, html, Html};
use crate::previews::CollapsePreview;

#[component(CollapsePage)]
pub(crate) fn collapse_page() -> Html {
  html! {
    <div class="CollapsePage page-container">
      <CollapsePreview/>
    </div>
  }
}