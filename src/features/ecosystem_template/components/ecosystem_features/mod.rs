use yew::{component, Html, Properties, html};
use yew_icons::{Icon};
use crate::features::ecosystem_template::components::section::Section;
use crate::features::ecosystem_template::types::EcosystemFeatureItem;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub features: Vec<EcosystemFeatureItem>,
}

#[component(EcosystemFeatures)]
pub fn ecosystem_features(props: &Props) -> Html {
  html! {
    <Section class="EcosystemFeatures" id="features" title="Features">
      <div class="ecosystem-features-list">
        {for props.features.iter().map(|feature| html! {
          <div class="ecosystem-feature">
            <div class="ecosystem-feature-icon-container">
              <Icon data={feature.icon.clone()} class="ecosystem-feature-icon"/>
            </div>
            <div class="ecosystem-feature-info">
              <h4 class="ecosystem-feature-title">{&feature.title}</h4>
              <p class="ecosystem-feature-description">{&feature.description}</p>
            </div>
          </div>
        })}
      </div>
    </Section>
  }
}