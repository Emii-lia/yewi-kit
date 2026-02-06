mod data;

use yew::{function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::docs::routes::DocsRoute;
use crate::components::{Button, ButtonVariant, CodePreview};
use crate::features::installation::data::get_steps;
use crate::types::Size;

#[function_component(InstallationSection)]
pub(crate) fn installation_section() -> Html {
  let steps = get_steps();

  html! {
    <div class="InstallationSection">
      <div class="InstallationSection__header">
        <h1 class="installation-title">{"Installation"}</h1>
        <p class="installation-subtitle">
          {r#"Setup and run Yewi-kit project in just a few simple steps."#}
        </p>
      </div>
      <div class="installation-steps">
        {for steps.iter().map(|step| {
          html! {
            <div class="installation-step" key={step.number}>
              <div class="installation-step__number">
                <div class="number-text">{step.number}</div>
              </div>
              <div class="installation-step__content">
                <h2 class="step-title">{step.title.clone()}</h2>
                <p class="step-description">{step.description.clone()}</p>
                {html! {
                 if step.command.clone() != "" {
                    <CodePreview code={step.command.clone()}/>
                  }
                }}
                {html! {
                  if let Some(hint) = step.hint.clone() {
                    <p class="step-hint">{hint}</p>
                  }
                }}
              </div>
            </div>
          }
        })}
      </div>
      <div class="installation-source">
        <h2 class="source-title">{"From Source"}</h2>
        <p class="source-subtitle">{"Alternatively, you can install Yewi-kit directly from the source:"}</p>
        <CodePreview
          code={r#"git clone https://github.com/Emii-lia/yewi-cli.git
cd yewi-cli
cargo install --path ."#}
        />
      </div>
      <div class="installation-footer">
        <h3 class="i-footer-title">{"What's Next?"}</h3>
        <p class="i-footer-subtitle">{"Now that Yewi-kit is installed, explore the available components and start building your application."}</p>
        <div class="i-footer-actions">
          <Link<DocsRoute> to={DocsRoute::AvatarPage}>
            <Button variant={ButtonVariant::Secondary} size={Size::Small}>
              {"Explore Components"}
            </Button>
          </Link<DocsRoute>>
          <Link<DocsRoute> to={DocsRoute::QuickStartPage}>
            <Button variant={ButtonVariant::Secondary} size={Size::Small}>
              {"Quick Start Guide"}
            </Button>
          </Link<DocsRoute>>
        </div>
      </div>
    </div>
  }
}