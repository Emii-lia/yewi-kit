use yew::{component, Html, Properties, html};
use crate::components::code_preview::CodePreview;
use crate::features::ecosystem_template::components::section::Section;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub subtitle: String,
  pub codes: Vec<(Option<String>, String)>,
}

#[component(EcosystemExample)]
pub fn ecosystem_example(props: &Props) -> Html {
  html! {
    <Section class="EcosystemExample" id="quick-example" title="Quick Example">
      <p class="ecosystem-example-subtitle">{&props.subtitle}</p>
      <div class="ecosystem-example-codes">
        {for props.codes.iter().map(|(label, code)| html! {
          <div class="ecosystem-example-code-block">
            if let Some(l) = label {
              <h4 class="ecosystem-example-code-label">{l}</h4>
            }
            <CodePreview code={code.clone()} />
          </div>
        })}
      </div>
    </Section>
  }
}