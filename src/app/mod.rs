use yew::{function_component, html, Html};
use crate::previews::ButtonPreview;

#[function_component]
pub fn App() -> Html {
  html! {
    <div class="App">
      <ButtonPreview/>
    </div>
  }
}