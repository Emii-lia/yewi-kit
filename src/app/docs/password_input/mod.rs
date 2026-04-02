use yew::{component, html, Html};
use crate::previews::PasswordInputPreview;

#[component(PasswordInputPage)]
pub(crate) fn password_input_page() -> Html {
  html! {
    <div class="PasswordInputPage page-container">
      <PasswordInputPreview/>
    </div>
  }
}