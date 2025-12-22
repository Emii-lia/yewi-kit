use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub(crate) fn not_found() -> Html {
  html! {
    <div class="NotFound">
      <h1>{"404"}</h1>
    </div>
  }
}