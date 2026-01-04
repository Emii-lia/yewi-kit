use yew::{AttrValue, Callback, Classes, Event, FocusEvent, InputEvent, KeyboardEvent, NodeRef, Properties};
use crate::types::Size;

#[derive(Properties, PartialEq, Clone)]
pub struct PasswordInputProps {
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(Size::Medium)]
  pub input_size: Size,
  #[prop_or_default]
  pub node_ref: NodeRef,
  #[prop_or_default]
  pub errors: Vec<String>,
  #[prop_or_default]
  pub placeholder: AttrValue,
  #[prop_or_default]
  pub value: Option<AttrValue>,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or_default]
  pub id: Option<AttrValue>,
  #[prop_or_default]
  pub name: Option<AttrValue>,
  #[prop_or_default]
  pub readonly: bool,
  #[prop_or_default]
  pub required: bool,
  #[prop_or_default]
  pub autofocus: bool,
  #[prop_or_default]
  pub autocomplete: Option<AttrValue>,
  #[prop_or_default]
  pub min: Option<AttrValue>,
  #[prop_or_default]
  pub max: Option<AttrValue>,
  #[prop_or_default]
  pub step: Option<AttrValue>,
  #[prop_or_default]
  pub pattern: Option<AttrValue>,
  #[prop_or_default]
  pub maxlength: Option<AttrValue>,
  #[prop_or_default]
  pub minlength: Option<AttrValue>,
  #[prop_or_default]
  pub tabindex: Option<AttrValue>,
  #[prop_or_default]
  pub oninput: Callback<InputEvent>,
  #[prop_or_default]
  pub onchange: Callback<Event>,
  #[prop_or_default]
  pub onblur: Callback<FocusEvent>,
  #[prop_or_default]
  pub onfocus: Callback<FocusEvent>,
  #[prop_or_default]
  pub onkeydown: Callback<KeyboardEvent>,
  #[prop_or_default]
  pub onkeyup: Callback<KeyboardEvent>,
  #[prop_or_default]
  pub onkeypress: Callback<KeyboardEvent>
}