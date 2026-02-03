use yew::{classes, function_component, html, Html};
use crate::previews::DropdownPreview;

#[function_component(DropdownPage)]
pub(crate) fn dropdown_page() -> Html {
  html! {
    <div class={classes!("DropdownPage", "page-container")}>
      <DropdownPreview/>
    </div>
  }
}