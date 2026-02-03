use yew::{function_component, html, Html};
use crate::previews::DividerPreview;

#[function_component(DividerPage)]
pub(crate) fn divider_page() -> Html {
  html! {
    <div class="DividerPage page-container">
      <DividerPreview/>
    </div>
  }
}