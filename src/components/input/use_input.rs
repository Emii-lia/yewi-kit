use yew::{hook, Properties};
use crate::types::Size;

#[derive(Properties, PartialEq)]
pub struct HookParams {
  pub size: Size
}

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub size: String
}

#[hook]
pub(crate) fn use_input(params: HookParams) -> HookResponse {
  let HookParams { size } = params;
  let size = match size {
    Size::Small => "sm",
    Size::Medium => "md",
    Size::Large => "lg"
  };

  HookResponse {
    size: size.to_string()
  }
}