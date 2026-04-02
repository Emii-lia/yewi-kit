use yew::{classes, component, html, Html};
use crate::previews::DropdownPreview;

#[component(DropdownPage)]
pub(crate) fn dropdown_page() -> Html {
  html! {
    <div class={classes!("DropdownPage", "page-container")}>
      <DropdownPreview/>
    </div>
  }
}