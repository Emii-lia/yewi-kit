use yew::{function_component, html, Html};
use crate::previews::AvatarPreview;

#[function_component(AvatarPage)]
pub(crate) fn avatar_page() -> Html {
  html! {
    <div class="AvatarPage page-container">
      <AvatarPreview/>
    </div>
  }
}