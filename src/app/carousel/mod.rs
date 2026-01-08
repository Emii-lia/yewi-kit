use yew::{classes, function_component, html, Html};
use crate::previews::CarouselPreview;

#[function_component(CarouselPage)]
pub(crate) fn carousel_page() -> Html {
  html! {
    <div class={classes!("CarouselPage", "page-container")}>
      <CarouselPreview/>
    </div>
  }
}