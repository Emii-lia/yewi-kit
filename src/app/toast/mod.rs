use yew::{classes, function_component, html, Html};
use crate::previews::ToastPreview;

#[function_component(ToastPage)]
pub(crate) fn toast_page() -> Html {
  html! {
    <div class={classes!("ToastPage", "page-container")}>
      <ToastPreview/>
    </div>
  }
}