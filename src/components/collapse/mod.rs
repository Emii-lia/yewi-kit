mod types;

use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};
pub(crate) use types::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub summary: AttrValue,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(CollapseVariant::Focus)]
  pub variant: CollapseVariant,
}
#[function_component(Collapse)]
pub(crate) fn collapse(props: &Props) -> Html {
  match props.variant {
    CollapseVariant::Toggle => {
      html! {
        <details class={classes!("Collapse", "toggle")}>
          <summary class="collapse-trigger">{&props.summary}</summary>
          <div class={classes!("collapse-content", &props.class)}>
            {props.children.clone()}
          </div>
        </details>
      }
    },
    CollapseVariant::Focus => {
      html! {
        <div class="Collapse" tabindex={"1"}>
          <div class="collapse-trigger">{&props.summary}</div>
          <div class={classes!("collapse-content", &props.class)}>
            {props.children.clone()}
          </div>
        </div>
      }
    }
  }
}