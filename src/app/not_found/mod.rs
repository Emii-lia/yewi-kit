use yew::{component, html, Html};

#[component(NotFound)]
pub(crate) fn not_found() -> Html {
  html! {
    <main class="NotFound">
      <div class="not-found-container">
        <div class="not-found-header">
          <h2 class="not-found-title">
            {"404"}
          </h2>
          <div class="not-found-line" />
        </div>

        <h2 class="not-found-subtitle">
          {"Page Not Found"}
        </h2>
      </div>
    </main>
  }
}