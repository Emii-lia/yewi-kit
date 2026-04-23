use std::collections::BTreeMap;
use yew::{hook, use_state, Callback};
use crate::features::theme_lab::provider::ThemeLabContextType;
use crate::types::theme::ThemeColor;
use crate::utils::shades::{shades_of, ShadeKey};

#[hook]
pub fn use_theme_lab_provider() -> ThemeLabContextType {
  let theme = use_state(|| ThemeColor::Slate);

  let set_theme = {
    let theme = theme.clone();
    Callback::from(move |new_theme: ThemeColor| {
      let theme = theme.clone();
      theme.set(new_theme);
    })
  };

  ThemeLabContextType {
    theme: (*theme).clone(),
    set_theme,
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ThemeLabHookParams {
  pub theme: ThemeColor
}

#[hook]
pub fn use_theme_lab(params: ThemeLabHookParams) -> BTreeMap<String, String> {
  let mut result = BTreeMap::new();
  match params.theme {
    ThemeColor::Custom(hex) => {
      let shades = shades_of(&hex).unwrap();
      for (shade_key, hex) in shades {
        match shade_key {
          ShadeKey::U(key) => {
            result.insert(format!("--tl-primary-{}", key), hex);
          },
          ShadeKey::Default => {
            result.insert("--tl-primary".to_string(), hex);
          }
        }
      }
    },
    _ => {}
  }

  result
}

