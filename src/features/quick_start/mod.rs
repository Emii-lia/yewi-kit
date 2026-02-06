mod data;

use yew::{function_component, html, Html};
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::{Button, ButtonVariant, Card, CardContent, CardDescription, CardTitle, CodePreview};
use crate::features::quick_start::data::get_common_components;
use crate::types::Size;

#[function_component(QuickStart)]
pub fn quick_start() -> Html {
  let common_components = get_common_components();
  html! {
    <div class="QuickStart">
      <div class="QuickStart__header">
        <h2 class="page-title">{"Quick Start"}</h2>
        <p class="page-subtitle">{"Learn the basics of building with Yewi-kit, from project setup to your first component."}</p>
      </div>
      <div class="QuickStart__section">
        <h3 class="qs-section-title">{"Prerequisites"}</h3>
        <div class="qs-section-content">
          <ul class="prerequisite-list">
            <li class="prerequisite-item">
              <Icon data={IconData::LUCIDE_CHECK} class="check-icon" />
              {"Rust 1.84.0 or later"}
            </li>
            <li class="prerequisite-item">
              <Icon data={IconData::LUCIDE_CHECK} class="check-icon" />
              {"Cargo (comes with Rust)"}
            </li>
            <li class="prerequisite-item">
              <Icon data={IconData::LUCIDE_CHECK} class="check-icon" />
              {"Trunk (for building and running Yew applications)"}
            </li>
            <li class="prerequisite-item">
              <Icon data={IconData::LUCIDE_CHECK} class="check-icon" />
              {"Node.js (for tailwindcss and sass integration)"}
            </li>
          </ul>
        </div>
      </div>
      <div class="QuickStart__section">
        <h3 class="qs-section-title">{"Basic usage"}</h3>
        <div class="qs-section-content">
          <p class="qs-section-content-text">
            {"Here's a simple example of how to use Yewi-kit components in your Yew application:"}
          </p>
          <CodePreview code={r#"use yew::prelude::*;
use crate::components::{Button, Card};

#[function_component]
fn App() -> Html {
  let onclick = Callback::from(|_| {
    web_sys::window()
      .unwrap()
      .alert_with_message("Button clicked!")
      .unwrap();
  });

  html! {
    <div class="p-4">
      <Card>
        <h1>{"Welcome to Yewi-kit"}</h1>
        <Button {onclick}>
          {"Click me"}
        </Button>
      </Card>
    </div>
  }
}
          "#}/>
        </div>
      </div>
      <div class="QuickStart__section">
        <h3 class="qs-section-title">{"Common components"}</h3>
        <div class="qs-section-content">
          <div class="common-components-list">
            {for common_components.iter().map(|card| {
              html! {
                <Link<DocsRoute>
                  to={card.to.clone()}
                  classes="component-item"
                  key={card.title.clone()}
                >
                  <Card class="component-item-card">
                    <CardContent>
                      <CardTitle>{card.title.clone()}</CardTitle>
                      <CardDescription>{card.description.clone()}</CardDescription>
                    </CardContent>
                  </Card>
                </Link<DocsRoute>>
              }
            })}
          </div>
        </div>
      </div>
      <div class="QuickStart__section">
        <h3 class="qs-section-title">{"Project Structure"}</h3>
        <div class="qs-section-content">
          <p class="qs-section-content-text">{"Yewi-kit projects follow a consistent, scalable architecture:"}</p>
          <div class="qs-code-block">
            <pre class="qs-code">
              <code>{r#"src/
├── app/                 # Pages and routes
├── components/          # Reusable UI components
├── features/            # Feature modules (auth, dashboard, etc.)
├── styles/              # Global and component styles
│   ├── components.scss  # Component-specific styles
│   ├── features.scss    # Feature-level styles
│   ├── main.scss        # Global styles
│   └── global.css       # Compiled css
├── main.rs              # Application entry point
└── Cargo.toml"#}</code>
            </pre>
          </div>
          <ul class="ps-list">
            <li>
              <strong class="ps-item-st">{"src/app: "}</strong>{"Page components and route definitions."}
            </li>
            <li>
              <strong class="ps-item-st">{"src/components: "}</strong>{"Reusable components from Yewi-kit or custom components."}
            </li>
            <li>
              <strong class="ps-item-st">{"src/features: "}</strong>{"Feature modules that contain related components, state, and logic."}
            </li>
            <li>
              <strong class="ps-item-st">{"src/styles: "}</strong>{"SCSS stylesheets organized by scope."}
            </li>
          </ul>
        </div>
      </div>
      <div class="QuickStart__section">
        <h3 class="qs-section-title">{"Styling with SCSS"}</h3>
        <div class="qs-section-content">
          <p class="styling-text">
            {"Yewi-kit supports SCSS for advanced styling. Import your component and feature styles in the designated files using the"}
            <code class="code-highlight">{"@use"}</code> {"directive:"}
          </p>
          <div class="styling-code-section">
            <p class="scs-text">{"src/styles/components.scss"}</p>
            <div class="qs-code-block">
              <pre class="qs-code">
                <code>
                  {r#"@use '../components/button';
@use '../components/card';
@use '../components/input';
@use '../components/avatar';"#}
                </code>
              </pre>
            </div>
          </div>
          <div class="styling-code-section">
            <p class="scs-text">{"src/styles/features.scss"}</p>
            <div class="qs-code-block">
              <pre class="qs-code">
                <code>
                  {r#"@use '../features/login';
@use '../features/dashboard';
@use '../features/settings';"#}
                </code>
              </pre>
            </div>
          </div>
          <p class="styling-hint">
            {"Keep your component styles in "}
            <code class="code-highlight">{"src/styles/components.scss"}</code>
            {"nd feature-level styles in "}
            <code class="code-highlight">{"src/styles/features.scss"}</code>
            {". This maintains a clean, organized stylesheet architecture."}
          </p>
        </div>
      </div>
      <div class="QuickStart__footer">
        <h3 class="qs-footer-title">{"Need help?"}</h3>
        <p class="qs-footer-subtitle">{"Check out the component documentation for detailed examples and API documentation."}</p>
        <div class="qs-footer-actions">
          <Link<DocsRoute> to={DocsRoute::AvatarPage}>
            <Button variant={ButtonVariant::Secondary} size={Size::Small}>
              {"View Components"}
            </Button>
          </Link<DocsRoute>>
          <Button
            variant={ButtonVariant::Secondary}
            size={Size::Small}
            href={"https://github.com/Emii-lia/yewi-kit"}
          >
            {"Github"}
          </Button>
        </div>
      </div>
    </div>
  }
}