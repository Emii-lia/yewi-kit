use yew::{Callback, MouseEvent, Properties, prelude::*};
use crate::components::ButtonClick;
use crate::types::{Size, ButtonVariant};

#[derive(Properties, PartialEq)]
pub struct HookParams {
  pub variant: ButtonVariant,
  pub size: Size,
  pub onclick: Option<ButtonClick>
}

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub variant: String,
  pub size: String,
  pub on_click: Callback<MouseEvent>
}

#[hook]
pub(crate) fn use_button(params: HookParams) -> HookResponse {
  let HookParams { variant, size, onclick } = params;
  let onclick = onclick;
  let on_click = Callback::from(move |e: MouseEvent| {
    if let Some(handler) = &onclick {
      match handler {
        ButtonClick::Mouse(cb) => cb.emit(e),
        ButtonClick::Simple(cb) => cb.emit(())
      }
    }
  });

  let variant = match variant {
    ButtonVariant::Primary => "primary",
    ButtonVariant::Secondary => "secondary",
    ButtonVariant::Tertiary => "tertiary",
    ButtonVariant::Danger => "danger",
    ButtonVariant::Success => "success",
  };

  let size = match size {
    Size::Small => "btn-sm",
    Size::Medium => "btn-md",
    Size::Large => "btn-lg",
  };

  HookResponse {
    variant: variant.to_string(),
    size: size.to_string(),
    on_click
  }
}