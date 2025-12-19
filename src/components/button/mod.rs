mod use_button;

use yew::{classes, function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};
use crate::components::button::use_button::{use_button, HookParams, HookResponse};
use crate::types::{ButtonSize, ButtonVariant};

#[derive(Clone, PartialEq)]
pub enum ButtonClick {
  Mouse(Callback<MouseEvent>),
  Simple(Callback<()>)
}
#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Html,
  #[prop_or(ButtonVariant::Primary)]
  pub variant: ButtonVariant,
  #[prop_or_default]
  pub class: String,
  #[prop_or_default]
  pub onclick: Option<ButtonClick>,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or(ButtonSize::Medium)]
  pub size: ButtonSize,
  #[prop_or_default]
  pub href: AttrValue,
  #[prop_or_default]
  pub is_loading: bool,
}
#[function_component]
pub(crate) fn Button(Props {
  href,
  children,
  is_loading,
  class,
  disabled,
  onclick,
  variant,
  size
}: &Props) -> Html {
  
  let HookResponse {
    variant,
    size,
    on_click
  } = use_button(HookParams {
    size: size.clone(),
    variant: variant.clone(),
    onclick: onclick.clone()
  });

  html! {
    if Some(href) != None && href != "" {
      <a
        href={href.clone()}
        class={classes!("Button", class, variant, size, if is_loading.clone() { "loading" } else { "" })}
        disabled={disabled.clone() || is_loading.clone()}
      >
        {children.clone()}
      </a>
    } else {
      <button
        class={classes!("Button", class, variant, size, if is_loading.clone() { "loading" } else { "" })}
        disabled={disabled.clone() || is_loading.clone()}
        onclick={on_click}
      >
        {
          html! {
            if is_loading.clone() {
              <div class="spinner-container">
                <span class="spinner"></span>
              </div>
            } else {
              {children.clone()}
            }
          }
        }
      </button>
    }
  }
}