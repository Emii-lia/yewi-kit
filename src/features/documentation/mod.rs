use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::{Button, ButtonVariant, Card, CardContent, CardDescription, CardTitle};
use crate::types::Size;

#[function_component(Documentation)]
pub fn documentation() -> Html {
  html! {
    <div class="Documentation">
      <div class="doc-header">
        <h2 class="page-title">{ "Documentation" }</h2>
        <p class="page-subtitle">{ "Welcome to the Yewi-kit documentation. Learn how to build beautiful web applications with Yewi component library." }</p>
      </div>
      <div class="doc-section">
        <h3 class="doc-section-title">{ "Getting Started" }</h3>
        <div class="doc-section-content">
          <div class="get-started">
            <Link<DocsRoute> to={DocsRoute::Installation} classes="gs-link">
              <Card class="gs-card">
                <CardContent>
                  <CardTitle>{"Installation"}</CardTitle>
                  <CardDescription>{"Get started with Yewi-kit installation and setup"}</CardDescription>
                </CardContent>
              </Card>
            </Link<DocsRoute>>
            <Link<DocsRoute> to={DocsRoute::QuickStartPage} classes="gs-link">
              <Card class="gs-card">
                <CardContent>
                  <CardTitle>{"Quick Start"}</CardTitle>
                  <CardDescription>{"Learn the basics and build your firt app in minutes"}</CardDescription>
                </CardContent>
              </Card>
            </Link<DocsRoute>>
          </div>
        </div>
      </div>
      <div class="doc-section">
        <h3 class="doc-section-title">{ "Components" }</h3>
        <div class="doc-section-content">
          <p class="doc-section-text">{"Explore all available components in Yewi-kit. Each component is fully documented with examples."}</p>
          <div class="components-list">
            {
              for DocsRoute::iter()
              .filter(|r| *r != DocsRoute::Docs && *r != DocsRoute::Installation && *r != DocsRoute::QuickStartPage)
              .map(|route| {
                html! {
                  <Link<DocsRoute> to={route.clone()} classes="comp-link">
                    <Card class="comp-card">
                      <CardContent>
                        <CardTitle>{ route.clone().to_string() }</CardTitle>
                      </CardContent>
                    </Card>
                  </Link<DocsRoute>>
                }
              })
            }
         </div>
        </div>
      </div>
      <div class="doc-footer">
        <h3 class="doc-footer-title">{"Need help?"}</h3>
        <p class="doc-footer-subtitle">{"Check out our GitHub repository for issues, and more information."}</p>
        <div class="doc-footer-actions">
          <Button
            variant={ButtonVariant::Secondary}
            size={Size::Small}
            href={"https://github.com/Emii-lia/yewi-kit"}
          >
            {"View on Github"}
          </Button>
        </div>
      </div>
    </div>
  }
}