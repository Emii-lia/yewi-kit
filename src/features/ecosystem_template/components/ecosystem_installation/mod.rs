use yew::{component, Html, Properties, html};
use crate::components::code_preview::CodePreview;
use crate::features::ecosystem_template::components::section::Section;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub subtitle: String,
  pub codes: Vec<String>,
}

#[component(EcosystemInstallation)]
pub fn ecosystem_installation(props: &Props) -> Html {
  html! {
    <Section class="EcosystemInstallation" id="installation" title="Installation">
      <p class="ecosystem-installation-subtitle">{&props.subtitle}</p>
      <div class="ecosystem-installation-codes">
        {for props.codes.iter().map(|code| html! {
          <CodePreview code={code.clone()} key={code.clone()}/>
        })}
      </div>
    </Section>
  }
}