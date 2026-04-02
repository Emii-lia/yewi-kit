mod data;
pub mod hooks;

use yew::{classes, component, html, Callback, Html};
use crate::components::CodePreview;
use crate::features::hooks::use_theme_section;
use crate::features::theme_section::data::{ThemeColor};

#[component(ThemeSection)]
pub fn theme_section() -> Html {
  let colours = ThemeColor::iter()
    .filter(|c| c != &"Custom".to_string())
    .map(|c| c.to_string()).collect::<Vec<String>>();

  let (
    selected,
    color_input_ref,
    on_select,
    on_change,
    on_input_click
  ) = use_theme_section();


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
              <div
                key={colour.clone()}
                class="colour-item"
                onclick={{
                  let theme_color = ThemeColor::from_str(colour.clone().as_str());
                  let on_select = on_select.clone();
                  Callback::from(move |_| on_select.emit(theme_color.clone()))
                }}
              >
                <div
                  class={classes!(
                    "colour-square", colour.clone().to_lowercase(),
                    (selected.to_string().to_lowercase() == colour.to_lowercase()).then_some("selected")
                  )}
                />
                <span class="colour-label">{colour}</span>
              </div>
            }
          })}
          <div class="colour-item">
            <input
              type="color"
              ref={color_input_ref.clone()}
              onchange={on_change.clone()}
              value={selected.to_string()}
              class={"colour-input"}
            />
            <div
              class={classes!(
                "colour-square", "custom",
                matches!(selected, ThemeColor::Custom(_)).then_some("selected")
              )}
              style={format!("--colour: {}", match &selected {
                ThemeColor::Custom(c) => c.clone(),
                _ => "#282828".to_string()
              })}
              onclick={on_input_click.clone()}
            />
            <span class="colour-label">{"Custom"}</span>
          </div>
        </div>
        <div class={"theme-section-command"}>
          <CodePreview
            code={format!("yewi new my-project --theme {}", selected.to_string().to_lowercase())}
            class={"theme-command"}
          />
          <span class={"theme-command-separator"}>{"Or update with"}</span>
          <CodePreview
            code={format!("yewi set --theme {}", selected.to_string().to_lowercase())}
            class={"theme-command"}
          />
        </div>

      </div>
    </div>
  }
}