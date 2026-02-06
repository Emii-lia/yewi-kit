use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub(crate) fn not_found() -> Html {
  html! {
    <main class="NotFound">
      <div class="not-found-container">
        <div class="not-found-header">
          <h1 class="not-found-title">
            {"404"}
          </h1>
          <div class="not-found-line" />
        </div>

        <h2 class="not-found-subtitle">
          {"Page Not Found"}
        </h2>
      </div>
    </main>
  }
}