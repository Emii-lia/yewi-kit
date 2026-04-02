use yew::{classes, component, html, Html};
use crate::previews::ToastPreview;

#[component(ToastPage)]
pub(crate) fn toast_page() -> Html {
  html! {
    <div class={classes!("ToastPage", "page-container")}>
      <ToastPreview/>
    </div>
  }
}