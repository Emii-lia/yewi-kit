use web_sys::HtmlInputElement;
use yew::{hook, use_memo, use_node_ref, use_state, Callback, NodeRef, TargetCast};
use crate::types::theme::{ThemeColor, ThemeGradient};

#[hook]
pub fn use_theme_section() -> (
  ThemeColor, 
  NodeRef,
  Callback<ThemeColor>,
  Callback<web_sys::Event>,
  Callback<web_sys::MouseEvent>,
  Option<ThemeGradient>
) {
  let selected_theme = use_state(|| ThemeColor::Slate);
  let color_input_ref = use_node_ref();
  
  let on_theme_select = {
    let selected_theme = selected_theme.clone();
    Callback::from(move |theme: ThemeColor| {
      selected_theme.set(theme);
    })
  };
  
  let on_color_change = {
    let selected_theme = selected_theme.clone();
    Callback::from(move |e: web_sys::Event| {
      if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
        let color_value = input.value();
        selected_theme.set(ThemeColor::Custom(color_value));
      }
    })
  };
  
  let on_color_click = {
    let color_input_ref = color_input_ref.clone();
    Callback::from(move |_| {
      if let Some(input) = color_input_ref.cast::<HtmlInputElement>() {
        input.click();
      }
    })
  };

  let theme_gradient = {
    let selected_theme = selected_theme.clone();
    use_memo(
      selected_theme,
      |theme| {
        let theme = theme.clone();
        ThemeGradient::from_theme(&(*theme))
      }
    )
  };

  (
    (*selected_theme).clone(), 
    color_input_ref,
    on_theme_select,
    on_color_change,
    on_color_click,
    (*theme_gradient).clone()
  )
}