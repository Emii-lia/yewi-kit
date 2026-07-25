use yew::{component, Html, Properties, html};
use crate::features::ecosystem_template::components::section::Section;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  pub ecosystem: String,
  pub why: String,
}

#[component(EcosystemPurpose)]  
pub fn ecosystem_purpose(props: &Props) -> Html {
  html! {
    <Section class="EcosystemPurpose" id="purpose" title={format!("Why {}?", props.ecosystem)}>
      <p class="ecosystem-purpose-why">{&props.why}</p>
    </Section>
  }
}