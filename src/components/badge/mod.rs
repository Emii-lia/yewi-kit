mod types;
mod hooks;

use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};
pub(crate) use types::*;
use crate::components::badge::hooks::{use_badge, HookParams, HookResponse};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  #[prop_or_default]
  pub label: AttrValue,
  #[prop_or(BadgeVariant::Default)]
  pub variant: BadgeVariant,
  #[prop_or(BadgeColor::None)]
  pub color: BadgeColor,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub rounded: bool,
  #[prop_or_default]
  pub with_border: bool,
  #[prop_or_default]
  pub title: AttrValue,
}

#[function_component(Badge)]
pub(crate) fn badge(props: &Props) -> Html {
  let HookResponse { color, variant } = use_badge(HookParams {
    variant: props.variant.clone(),
    color: props.color.clone()
  });
  
  html! {
    <div class={classes!(
        "Badge",
        props.rounded.then_some("rounded"),
        props.with_border.then_some("with-border"),
        variant,
        color,
        &props.class
      )}
      title={if !props.title.is_empty() { &props.title } else { &props.label }}
    >
      {&props.label}
    </div>
  }
}