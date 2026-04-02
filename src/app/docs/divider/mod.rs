use yew::{component, html, Html};
use crate::previews::DividerPreview;

#[component(DividerPage)]
pub(crate) fn divider_page() -> Html {
  html! {
    <div class="DividerPage page-container">
      <DividerPreview/>
    </div>
  }
}