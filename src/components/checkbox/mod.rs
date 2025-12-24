mod types;
mod use_checkbox;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
use crate::components::checkbox::use_checkbox::{use_checkbox, HookParams, HookResponse};
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
  pub name: String,
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
  let color_class = format!("{:?}", props.color.clone()).to_lowercase();

  html! {
    <label
      for={props.id.clone()}
      class={classes!(
        "Checkbox",
        format!("{:?}", props.variant.clone()).to_lowercase(),
        format!("{:?}", props.size.clone()).to_lowercase(),
        color_class.clone(),
        props.disabled.clone().then_some("disabled"),
        props.checked.clone().then_some("checked"),
        props.class.clone(),
      )}
    >
      <input
        class={classes!(
          "checkbox-input",
          color_class.clone()
        )}
        type="checkbox"
        disabled={props.disabled.clone()}
        name={props.name.clone()}
        id={props.id.clone()}
        checked={props.checked.clone()}
        value={props.value.clone()}
        onchange={on_change.clone()}
      />
      {html! {
        if props.variant == CheckboxVariant::Toggle {
          <span class={classes!(
            "checkbox-toggle",
            color_class.clone()
          )}></span>
        }
      }}
      {html! {
        if !props.label.is_empty() {
          <span class="checkbox-label">{props.label.clone()}</span>
        }
      }}
    </label>
  }
}