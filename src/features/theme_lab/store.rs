use yew::{hook, use_context};
use crate::features::theme_lab::provider::ThemeLabContextType;

#[hook]
pub fn use_theme_lab_store() -> ThemeLabContextType {
  use_context::<ThemeLabContextType>()
    .expect("ThemeLabContextType not found")
}