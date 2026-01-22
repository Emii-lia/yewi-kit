mod hooks;
mod types;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, MouseEvent, Properties};
use yew_icons::{Icon, IconData};
use crate::components::button::hooks::{use_button, HookParams, HookResponse};
use crate::types::{Size};
pub use types::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: Html,
  #[prop_or(ButtonVariant::Primary)]
  pub variant: ButtonVariant,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub onclick: Callback<MouseEvent>,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or_default]
  pub href: AttrValue,
  #[prop_or_default]
  pub is_loading: bool,
  #[prop_or_default]
  pub icon: Option<IconData>
}
#[function_component(Button)]
pub(crate) fn button(props: &Props) -> Html {
  
  let HookResponse {
    variant,
    size,
    on_click
  } = use_button(HookParams {
    size: props.size.clone(),
    variant: props.variant.clone(),
    onclick: props.onclick.clone()
  });

  let icon = props.icon;
  let icon_size = match props.size {
    Size::Small => "1rem",
    Size::Medium => "1.25rem",
    Size::Large => "1.5rem",
  };

  html! {
    if Some(&props.href) != None && &props.href != "" {
      <a
        href={&props.href}
        class={classes!(
        "Button",
        variant,
        size,
        props.is_loading.then_some("loading"),
        &props.class,
        icon.is_some().then_some("with-icon")
      )}
        disabled={props.disabled || props.is_loading}
      >
        {html! {
          if let Some(icon) = icon {
            <Icon data={icon} width={icon_size.to_string()} height={icon_size.to_string()} />
          }
        }}
        {props.children.clone()}
      </a>
    } else {
      <button
        class={classes!(
          "Button",
          variant,
          size,
          props.is_loading.then_some("loading"),
          &props.class,
          icon.is_some().then_some("with-icon")
      )}
        disabled={props.disabled || props.is_loading}
        onclick={on_click}
      >
        {
          html! {
            if props.is_loading {
              <div class="spinner-container">
                <span class="spinner"></span>
              </div>
            } else {
              <>
                {html! {
                  if let Some(icon) = icon {
                    <Icon data={icon} width={icon_size.to_string()} height={icon_size.to_string()} />
                  }
                }}
                {props.children.clone()}
              </>
            }
          }
        }
      </button>
    }
  }
}