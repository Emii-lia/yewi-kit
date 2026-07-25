use yew::{component, Html, html};
use crate::features::ecosystem_template::components::ecosystem_example::EcosystemExample;
use crate::features::ecosystem_template::components::ecosystem_features::EcosystemFeatures;
use crate::features::ecosystem_template::components::ecosystem_header::EcosystemHeader;
use crate::features::ecosystem_template::components::ecosystem_installation::EcosystemInstallation;
use crate::features::ecosystem_template::components::ecosystem_integration::EcosystemIntegration;
use crate::features::ecosystem_template::components::ecosystem_overview::EcosystemOverview;
use crate::features::ecosystem_template::components::ecosystem_purpose::EcosystemPurpose;
use crate::features::ecosystem_template::types::{Ecosystem};

pub mod components;
pub mod types;

#[component(EcosystemTemplate)]
pub fn ecosystem_template<E>() -> Html
where
  E: Ecosystem + 'static,
{
  let header = E::header();
  let overview = E::overview();
  let features = E::features();
  let installation = E::installation();
  let examples = E::example();
  let purpose = E::purpose();

  html! {
    <main class="EcosystemTemplate">
      <EcosystemHeader
        title={header.title}
        description={header.description}
        tags={header.tags}
        github={header.github.clone()}
        crates={header.crates.clone()}
      />
      <EcosystemOverview
        problem={overview.problem.clone()}
        solution={overview.solution.clone()}
        benefits={overview.benefits}
      />
      <EcosystemFeatures
        features={features.features}
      />
      <EcosystemInstallation
        subtitle={installation.subtitle.clone()}
        codes={installation.codes}
      />
      <EcosystemExample
        subtitle={examples.subtitle}
        codes={examples.codes}
      />
      <EcosystemPurpose
        why={purpose.why}
        ecosystem={purpose.ecosystem}
      />
      <EcosystemIntegration/>
    </main>
  }
}