use yew::{html, Html, component, Properties, AttrValue};
use yew_icons::IconData;
use crate::components::badge::Badge;
use crate::components::button::{Button, ButtonVariant};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub title: AttrValue,
  #[prop_or_default]
  pub description: AttrValue,
  #[prop_or(vec![])]
  pub tags: Vec<&'static str>,
  #[prop_or_default]
  pub github: String,
  #[prop_or_default]
  pub crates: String
}

#[component(EcosystemHeader)]
pub fn ecosystem_header(props: &Props) -> Html {
  html! {
    <header class="EcosystemHeader">
      <h2 class="ecosystem-header-title">{&props.title}</h2>
      <p class="ecosystem-header-description">{&props.description}</p>
      <div class="ecosystem-header-tags">
        {for props.tags.iter().map(|tag| html! {
          <Badge
            key={tag.to_string()}
            label={(*tag).to_string()}
            with_border={true}
            class={"ecosystem-header-badge"}
          />
        })}
      </div>
      <div class="ecosystem-header-actions">
        {if !props.crates.is_empty() {
          html! {
            <Button
              class="ecosystem-header-action-button"
              href={props.crates.clone()}
              // variant={ButtonVariant::Secondary}
              icon={IconData::LUCIDE_GLOBE}
            >
              {"Crates.io"}
            </Button>
          }
        } else {
          html! {}
        }}
        {if !props.github.is_empty() {
          html! {
            <Button
              class="ecosystem-header-action-button"
              href={props.github.clone()}
              variant={ButtonVariant::Secondary}
              icon={IconData::LUCIDE_GITHUB}
            >
              {"GitHub"}
            </Button>
          }
        } else {
          html! {}
        }}
      </div>
    </header>
  }
}