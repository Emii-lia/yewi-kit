mod data;

use yew::{classes, function_component, html, Html};
use crate::features::theme_section::data::get_theme_colours;

#[function_component(ThemeSection)]
pub fn theme_section() -> Html {
  let colours = get_theme_colours();

  html! {
    <div class="ThemeSection">
      <div class="theme-section-container">
        <div class="theme-section-text">
          <h2 class="theme-section-title">{"Choose your look"}</h2>
          <p class="theme-section-description">
            {r#"Pick a colour theme during project creation and start with a UI that already feels right.
            Themes are clean, neutral, and designed to work beautifully with Yewi components."#}
          </p>
        </div>
        <div class="colour-list">
          {for colours.iter().cloned().map( |colour| {
            html! {
              <div key={colour} class="colour-item">
                <div
                  class={classes!("colour-square", colour.to_lowercase())}
                />
                <span class="colour-label">{colour}</span>
              </div>
            }
          })}
        </div>
      </div>
    </div>
  }
}