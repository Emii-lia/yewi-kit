use yew::{function_component, html, Html};
use crate::components::Button;
use crate::types::{ButtonSize, ButtonVariant};

#[function_component]
pub fn App() -> Html {
  html! {
    <div class="App">
      <Button
        variant={ButtonVariant::Primary}
      >
        {"My Button"}
      </Button>
    </div>
  }
}