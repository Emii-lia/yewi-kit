use yew::{function_component, html, Html};
use crate::previews::PasswordInputPreview;

#[function_component(PasswordInputPage)]
pub(crate) fn password_input_page() -> Html {
  html! {
    <div class="PasswordInputPage page-container">
      <PasswordInputPreview/>
    </div>
  }
}