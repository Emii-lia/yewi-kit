use yew::{component, Html, Properties, html};
use crate::features::ecosystem_template::components::section::Section;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub problem: Html,
  #[prop_or_default]
  pub solution: Html,
  #[prop_or_default]
  pub benefits: Vec<String>
}

#[component(EcosystemOverview)]
pub fn ecosystem_overview(props: &Props) -> Html {
  html! {
    <Section class="EcosystemOverview" id="overview" title={"Overview"}>
      <div class="ecosystem-overview-paragraph">{props.problem.clone()}</div>
      <div class="ecosystem-overview-paragraph">{props.solution.clone()}</div>
      <div class="ecosystem-overview-benefits">
        <h4 class="ecosystem-overview-benefits-title">{"Benefits"}</h4>
        <ul class="ecosystem-overview-benefits-list">
          {for props.benefits.iter().map(|benefit| html! {
            <li class="ecosystem-overview-benefit">{benefit}</li>
          })}
        </ul>
      </div>
    </Section>
  }
}