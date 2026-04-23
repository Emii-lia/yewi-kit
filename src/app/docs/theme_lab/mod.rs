use yew::{component, html, Html};
use crate::features::theme_lab::provider::ThemeLabProvider;
use crate::features::theme_lab::ThemeLab;

#[component(ThemeLabPage)]
pub fn theme_lab_page() -> Html {
  html! {
    <div class="ThemeLabPage page-container">
      <ThemeLabProvider>
        <ThemeLab/>
      </ThemeLabProvider>
    </div>
  }
}