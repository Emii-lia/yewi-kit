use yew::{classes, function_component, html, Html};
use crate::previews::link_preview::LinkPreview;

#[function_component(LinkPage)]
pub fn link_page() -> Html {
  html! {
    <div class={classes!("LinkPage", "page-container")}>
      <LinkPreview/>
    </div>
  }
}