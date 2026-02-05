use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::Button;
use crate::types::Size;

#[function_component(HomeCta)]
pub fn home_cta() -> Html {
  html! {
    <div class="HomeCta">
      <div class="HomeCta__container">
        <h2 class="home-cta-title">{"Ready to ship?"}</h2>
        <p class="home-cta-description">
          {r#"Start building beautiful, type-safe web applications with Yewi-kit. Join developers who are shipping faster with Rust."#}
        </p>
        <Link<DocsRoute> to={DocsRoute::Docs}>
          <Button size={Size::Large}>
            {"Get Started"}
          </Button>
        </Link<DocsRoute>>
      </div>
    </div>
  }
}