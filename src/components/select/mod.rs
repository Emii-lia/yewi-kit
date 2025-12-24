mod types;

pub use types::*;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
use yew::html::onchange::Event;
use crate::types::Size;

#[derive(Properties, PartialEq ,Clone)]
pub struct Props {
  #[prop_or_default]
  pub name: AttrValue,
  #[prop_or_default]
  pub label: AttrValue,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub value: AttrValue,
  #[prop_or_default]
  pub main_class: Classes,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or_default]
  pub on_change: Option<Callback<Event>>,
  #[prop_or_default]
  pub options: Vec<SelectOption>,
  #[prop_or_default]
  pub errors: Vec<String>,
}
#[function_component(Select)]
pub(crate) fn select(props: &Props) -> Html {
  html! {
    <div class={classes!("Select", props.main_class.clone())}>
      <select
        disabled={props.disabled.clone()}
        name={props.name.clone()}
        id={props.name.clone()}
        value={props.value.clone()}
        onchange={props.on_change.clone()}
        class={classes!(
          "select",
          props.class.clone(),
          format!("{:?}", props.size.clone()).to_lowercase()
        )}
      >
        {html! {
          if !props.label.clone().is_empty() {
            <option value="" disabled=true>{props.label.clone()}</option>
          }
        }}
       {for props.options.iter().map(|option| {
          let (label, value) = match option {
            SelectOption::Pair { label, value } => (label.clone(), value.clone()),
            SelectOption::Simple(value) => (value.clone(), value.clone()),
          };
          html! {
            <option key={value.clone().to_string()} value={value.clone()}>
              {label.clone()}
            </option>
          }
        })}
      </select>
      {
        html! {
          if !props.errors.is_empty() {
            <div class="select-errors">
              {for props.errors.iter().map(|error| html! {
                <span class="select-error">
                  {format!("{}. ",error.clone())}
                </span>
              })}
            </div>
        }
      }}
    </div>
  }
}