mod types;
mod hooks;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
use crate::components::checkbox::hooks::{use_checkbox, HookParams, HookResponse};
use crate::types::{Color, Size};
pub use types::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub label: AttrValue,
  #[prop_or_default]
  pub checked: bool,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or_default]
  pub name: AttrValue,
  #[prop_or_default]
  pub value: AttrValue,
  #[prop_or_default]
  pub on_change: Callback<bool>,
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or(Color::Blue)]
  pub color: Color,
  #[prop_or(CheckboxVariant::Default)]
  pub variant: CheckboxVariant,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub id: AttrValue,
}

#[function_component(Checkbox)]
pub(crate) fn checkbox(props: &Props) -> Html {
  let HookResponse { on_change } = use_checkbox(HookParams { onchange: props.on_change.clone() });
  let color_class = format!("{:?}", props.color).to_lowercase();
  let variant_class = format!("{:?}", props.variant).to_lowercase();
  let size_class = format!("{:?}", props.size).to_lowercase();

  html! {
    <label
      for={&props.id}
      class={classes!(
        "Checkbox",
        variant_class,
        size_class,
        &color_class,
        props.disabled.then_some("disabled"),
        props.checked.then_some("checked"),
        &props.class,
      )}
    >
      <input
        class={classes!(
          "checkbox-input",
          &color_class
        )}
        type="checkbox"
        disabled={props.disabled}
        name={&props.name}
        id={&props.id}
        checked={props.checked}
        value={&props.value}
        onchange={on_change.clone()}
      />
      {html! {
        if props.variant == CheckboxVariant::Toggle {
          <span class={classes!(
            "checkbox-toggle",
            &color_class
          )}></span>
        }
      }}
      {html! {
        if !props.label.is_empty() {
          <span class="checkbox-label">{&props.label}</span>
        }
      }}
    </label>
  }
}