use yew::{classes, component, html, Html};
use crate::previews::CarouselPreview;

#[component(CarouselPage)]
pub(crate) fn carousel_page() -> Html {
  html! {
    <div class={classes!("CarouselPage", "page-container")}>
      <CarouselPreview/>
    </div>
  }
}