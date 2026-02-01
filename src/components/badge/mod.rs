mod types;
mod hooks;

use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};
use yew_icons::{Icon, IconData};
pub(crate) use types::*;
use crate::components::badge::hooks::{use_badge, HookParams, HookResponse};
use crate::types::Color;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  #[prop_or_default]
  pub label: AttrValue,
  #[prop_or(BadgeVariant::Default)]
  pub variant: BadgeVariant,
  #[prop_or(Color::Primary)]
  pub color: Color,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub rounded: bool,
  #[prop_or_default]
  pub with_border: bool,
  #[prop_or_default]
  pub title: AttrValue,
  #[prop_or_default]
  pub icon: Option<IconData>,
}

#[function_component(Badge)]
pub(crate) fn badge(props: &Props) -> Html {
  let HookResponse { color, variant } = use_badge(HookParams {
    variant: props.variant.clone(),
    color: props.color.clone()
  });
  let icon = props.icon;
  
  html! {
    <div class={classes!(
        "Badge",
        props.rounded.then_some("badge-rounded"),
        props.with_border.then_some("badge-with-border"),
        variant,
        color,
        icon.is_some().then_some("badge-with-icon"),
        &props.class
      )}
      title={if !props.title.is_empty() { &props.title } else { &props.label }}
    >
      {html! {
        if let Some(icon_data) = icon {
          <Icon data={icon_data} width={"1rem".to_string()} height={"1rem".to_string()} />
        }
      }}
      {&props.label}
    </div>
  }
}