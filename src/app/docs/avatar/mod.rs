use yew::{component, html, Html};
use crate::previews::AvatarPreview;

#[component(AvatarPage)]
pub(crate) fn avatar_page() -> Html {
  html! {
    <div class="AvatarPage page-container">
      <AvatarPreview/>
    </div>
  }
}