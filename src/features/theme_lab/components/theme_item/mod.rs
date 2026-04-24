pub mod hooks;

use web_sys::MouseEvent;
use yew::{classes, component, html, Callback, Html, Properties};
use yew_icons::{Icon, IconData};
use crate::features::theme_lab::components::theme_item::hooks::{use_theme_item, HookParams, HookResponse};
use crate::features::theme_lab::provider::ThemeLabContextType;
use crate::features::theme_lab::store::use_theme_lab_store;
use crate::types::theme::ThemeColor;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or(ThemeColor::Slate)]
  pub theme: ThemeColor,
}
#[component(ThemeItem)]
pub fn theme_item(props: &Props) -> Html {
  let ThemeLabContextType {
    theme: selected_theme,
    set_theme
  } = use_theme_lab_store();

  let HookResponse {
    onselect,
    onchange,
    value,
    selected
  } = use_theme_item(HookParams {
    onselect: set_theme.clone(),
    selected_theme: selected_theme.clone(),
    theme: props.theme.clone(),
  });

  match props.theme.clone() {
    ThemeColor::Custom(_) => {
      html! {
        <div class="ThemeItem">
          <label 
            class={classes!("theme-colour-square", "custom", selected.then_some("selected"))}
            style={format!("--custom-colour: {}", value.clone().unwrap_or("gray".to_string()))}
            for={"theme-item-colour-input"}
          >
            <Icon data={IconData::LUCIDE_PLUS} class={"theme-colour-plus"}/>
            <input
              type={"color"}
              id={"theme-item-colour-input"}
              class={"theme-colour-input"}
              value={value.clone().unwrap_or_default()}
              onchange={onchange.clone()}
            />
          </label>
          <span class="theme-colour-label">{props.theme.clone().to_string()}</span>
        </div>
      }
    },
    _ => {
      html! {
        <div 
          class={classes!("ThemeItem")}
          onclick={{
            let onselect = onselect.clone();
            let theme = props.theme.clone();
            Callback::from(move |_e: MouseEvent| onselect.emit(theme.clone()))
          }}
        >
          <div
            class={classes!(
              "theme-colour-square", props.theme.clone().to_string().to_lowercase(),
              selected.then_some("selected")
            )}
          />
          <span class="theme-colour-label">{props.theme.clone().to_string()}</span>
        </div>
      }
    }
  }
}