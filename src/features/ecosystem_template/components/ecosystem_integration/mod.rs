use yew::{component, Html, html};
use crate::components::code_preview::CodePreview;
use crate::features::ecosystem_template::components::section::Section;

#[component(EcosystemIntegration)]
pub fn ecosystem_integration() -> Html {
  html! {
    <Section class="EcosystemIntegration" id="integration" title="Integration">
      <p class="ecosystem-integration-title">{"Part of the Yewi ecosystem"}</p>
      <CodePreview
        code={r#"[Yewi CLI] -> Project scaffolding
[Yewi app templates] -> Project template
[Yewi Kit] -> UI Components
[Yewi SEO] -> Document head management
        "#}
        hide_copy={true}
      />
    </Section>
  }
}