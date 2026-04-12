use yew::{component, html, Html};
use crate::previews::sidebar_preview::SidebarPreview;

#[component(SidebarPage)]
pub fn sidebar_page() -> Html {
  html! {
    <div class={"SidebarPage page-container"}>
      <SidebarPreview/>
    </div>
  }
}