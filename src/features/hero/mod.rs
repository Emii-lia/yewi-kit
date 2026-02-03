use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::{Button, ButtonVariant};
use crate::types::Size;

#[function_component(Hero)]
pub fn hero() -> Html {
  html! {
    <div class="Hero">
        <div class="hero-container">
          <div class="hero-text">
            <h2 class="hero-title">
              {"Rust Powered"}
              <br/>
              <span class="hero-title__primary">
                {"Web components"}
              </span>
            </h2>
            <p class="hero-subtitle">
              {"A Rust-first UI kit and CLI for building clean, scalable Yew frontends. Fast to start, easy to grow."}
            </p>
          </div>
          <div class="hero-ctas">
            <Link<DocsRoute>
              to={DocsRoute::Installation}
            >
              <Button
                class={"hero-cta"}
                size={Size::Large}
              >
                {"Get Started"}
              </Button>
            </Link<DocsRoute>>
            <Link<DocsRoute>
              to={DocsRoute::ButtonPage}
            >
              <Button
                class={"hero-cta"}
                size={Size::Large}
                variant={ButtonVariant::Secondary}
              >
                {"View Components"}
              </Button>
            </Link<DocsRoute>>
          </div>
        </div>
      </div>
  }
}