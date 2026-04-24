use web_sys::{Event, MouseEvent};
use yew::{hook, use_memo, use_state, Callback, TargetCast};
use crate::types::theme::ThemeColor;

#[derive(Clone)]
pub(crate) struct HookParams {
  pub onselect: Callback<ThemeColor>,
  pub selected_theme: ThemeColor,
  pub theme: ThemeColor,
}

pub(crate) struct HookResponse {
  pub onchange: Callback<Event>,
  pub onselect: Callback<ThemeColor>,
  pub value: Option<String>,
  pub selected: bool,
}

#[hook]
pub fn use_theme_item(params: HookParams) -> HookResponse {
  let value = use_state(|| None::<String>);

  let onchange = {
    let onselect = params.onselect.clone();
    let value = value.clone();
    Callback::from(move |e: Event| {
      let target = e.target_unchecked_into::<web_sys::HtmlInputElement>();
      onselect.emit(ThemeColor::Custom(target.value()));
      value.set(Some(target.value()));
    })
  };

  let onselect = {
    let onselect = params.onselect.clone();
    let value = value.clone();
    Callback::from(move |color: ThemeColor| {
      onselect.emit(color.clone());
      match color.clone() {
        ThemeColor::Custom(hex) => {
          value.set(Some(hex));
        },
        _ => {
          value.set(None);
        }
      }
    })
  };
  
  let selected = {
    let value = value.clone();
    use_memo((
      params.selected_theme.clone(),
      params.theme.clone(),
      value
    ),
      |(selected_theme, theme, value)| {
        let value = value.clone();
        match selected_theme {
          ThemeColor::Custom(hex) => {
            (*value).clone().map_or(
              false,
              |v| v == hex.as_str()
            )
          }
          _ => {
            theme == selected_theme
          }
        }
      }
    )
  };

  HookResponse {
    onchange,
    onselect,
    value: (*value).clone(),
    selected: (*selected).clone(),
  }
}