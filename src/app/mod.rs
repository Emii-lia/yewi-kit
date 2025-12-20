use yew::{function_component, html, Html};
use crate::components::{Button, Input};
use crate::types::{ButtonVariant, Size};

#[function_component]
pub fn App() -> Html {
  html! {
    <div class="App">
      <Input
        placeholder="My Input"
        errors={vec!["Error 1".to_string(), "Error 2".to_string()]}
      />
      <Button
        variant={ButtonVariant::Primary}
      >
        {"My Button"}
      </Button>
    </div>
  }
}