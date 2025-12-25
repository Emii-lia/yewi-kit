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
    <div class={classes!("Select", &props.main_class)}>
      <select
        disabled={props.disabled}
        name={&props.name}
        id={&props.name}
        value={props.value.clone()}
        onchange={props.on_change.clone()}
        class={classes!(
          "select",
          &props.class,
          format!("{:?}", props.size).to_lowercase()
        )}
      >
        {html! {
          if !props.label.is_empty() {
            <option value="" disabled=true>{&props.label}</option>
          }
        }}
       {for props.options.iter().map(|option| {
          let (label, value) = match option {
            SelectOption::Pair { label, value } => (label.clone(), value.clone()),
            SelectOption::Simple(value) => (value.clone(), value.clone()),
          };
          html! {
            <option key={value.to_string()} value={value}>
              {label}
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
                  {format!("{}. ", error)}
                </span>
              })}
            </div>
        }
      }}
    </div>
  }
}