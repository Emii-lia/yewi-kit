pub(crate) mod components;
pub mod provider;
pub mod hooks;
pub mod store;

use yew::{classes, component, html, Html};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::button::{Button, ButtonVariant};
use crate::components::checkbox::{Checkbox, CheckboxVariant};
use crate::components::code_preview::CodePreview;
use crate::components::pagination::Pagination;
use crate::components::progress::{Progress, ProgressVariant};
use crate::components::radio::Radio;
use crate::components::tabs::{Tab, Tabs};
use crate::features::theme_lab::components::theme_item::ThemeItem;
use crate::features::theme_lab::hooks::{use_theme_lab, ThemeLabHookParams};
use crate::features::theme_lab::provider::ThemeLabContextType;
use crate::features::theme_lab::store::use_theme_lab_store;
use crate::types::Size;
use crate::types::theme::ThemeColor;
use crate::utils::shades::ShadeKey;

#[component(ThemeLab)]
pub fn theme_lab() -> Html {
  let ThemeLabContextType {
    theme,
    ..
  } = use_theme_lab_store();
  let shades = use_theme_lab(ThemeLabHookParams { theme: theme.clone() });

  html! {
    <div class="ThemeLab">
      <div class={"ThemeLab__header"}>
        <h2 class="tl-title">{"Theme Lab"}</h2>
        <p class={"tl-subtitle"}>{"Explore and experiment with themes. Pick a preset, create a custom one, and see how components adapt in real-time."}</p>
      </div>
      <div class={"ThemeLab__switcher"}>
        {for ThemeColor::iter().map(|theme| html! {
          <ThemeItem key={theme.to_string()} theme={ThemeColor::from_str(&theme)}/>
        })}
      </div>
      <div class={"ThemeLab__code"}>
        <CodePreview code={format!("yewi new my-project --theme {}", theme.clone().to_string().to_lowercase())}/>
        <CodePreview code={format!("yewi set --theme {}", theme.clone().to_string().to_lowercase())}/>
      </div>
      <div
        class={classes!(
          "ThemeLab__content",
          match theme {
            ThemeColor::Custom(_) => "".to_string(),
            _ => format!("theme-{}", theme.to_string().to_lowercase())
          }
        )}
        style={shades.iter().map(|(key, value)| format!("{}: {};", key, value)).collect::<String>()}
      >
        <div class={"tl-preview-container tl-component-preview"}>
          <h3 class={"tl-preview-title"}>
            {"Component Previews"}
          </h3>
          <p class={"tl-preview-description"}>
            {"Showing theme: "}<span class={"tl-current-theme"}>{theme.to_string()}</span>
          </p>
          <div class={"tl-preview-content"}>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Buttons"}</h4>
              <div class={"tl-preview-item-content"}>
                <Button class={"tl-button-primary"}>
                  {"Primary"}
                </Button>
                <Button class={"tl-button-secondary"} variant={ButtonVariant::Secondary}>
                  {"Secondary"}
                </Button>
              </div>
            </div>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Badges"}</h4>
              <div class={"tl-preview-item-content"}>
                <Badge label={"Default"} class={"tl-badge-default"} />
                <Badge label={"Filled"} class={"tl-badge-filled"} variant={BadgeVariant::Filled} />
                <Badge label={"Plain"} class={"tl-badge-plain"} variant={BadgeVariant::Plain} />
              </div>
            </div>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Inputs"}</h4>
              <div class={"tl-preview-item-content"}>
                <Checkbox label={"Checkbox"} class={"tl-checkbox"} checked=true/>
                <Checkbox label={"Toggle"} class={"tl-toggle"} variant={CheckboxVariant::Toggle} checked=true id={"tl-toggle"}/>
                <Radio label={"Radio"} class={"tl-radio"} checked=true/>
              </div>
            </div>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Progress"}</h4>
              <div class={"tl-preview-item-content"}>
                <Progress value={65} class={"tl-progress"}/>
                <Progress value={65} class={"tl-radial"} variant={ProgressVariant::Circular}/>
              </div>
            </div>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Tabs"}</h4>
              <div class={"tl-preview-item-content"}>
                <Tabs classes={"tl-tabs"}>
                  <Tab label={"Preview"} value={"preview"}>
                    <div/>
                  </Tab>
                  <Tab label={"Code"} value={"code"}>
                    <div/>
                  </Tab>
                </Tabs>
              </div>
            </div>
            <div class={"tl-preview-item"}>
              <h4 class={"tl-preview-item-title"}>{"Pagination"}</h4>
              <div class={"tl-preview-item-content"}>
                <Pagination
                  count={10}
                  current={3}
                  rounded=true
                  size={Size::Small}
                  class={"tl-pagination"}
                />
              </div>
            </div>
          </div>
        </div>
        <div class={"tl-preview-container tl-palette"}>
          <h3 class={"tl-preview-title"}>
            {"Colour Palette"}
          </h3>
          <div class={"tl-preview-content"}>
            <div class={"tl-palette-group"}>
              <h4 class={"tl-palette-title"}>
                {"Primary shades"}
              </h4>
              <div class={"tl-palette-grid"}>
                {for (vec![50, 100, 200, 300, 400, 500]).iter().map(|shade| html! {
                  <div class={classes!("tl-palette-item", format!("tl-palette-item-{}", shade))} key={shade.to_string()}/>
                })}
              </div>
            </div>
            <div class={"tl-palette-group"}>
              <h4 class={"tl-palette-title"}>
                {"Dark shades"}
              </h4>
              <div class={"tl-palette-grid"}>
                {for (vec![600, 700, 800, 900, 950]).iter().map(|shade| html! {
                  <div
                    class={classes!("tl-palette-item", format!("tl-palette-item-{}", shade))} key={shade.to_string()}
                    title={theme.clone().to_string()}
                  />
                })}
              </div>
            </div>
          </div>
        </div>
        <div class={"tl-preview-container tl-shades"}>
          <h3 class={"tl-preview-title"}>
            {"All Generated Shades"}
          </h3>
          <div class={"tl-shades-list"}>
            {for ShadeKey::shades().into_iter()
              .filter(|shade| shade != &ShadeKey::Default)
              .map(|shade| html! {
              <div
                class={"tl-shade-item"}
                title={shades.get(&format!("--tl-primary-{}", shade.clone().to_string())).unwrap_or(&theme.clone().to_string().to_lowercase()).to_string()}
              >
                <div
                  class={classes!("tl-shade-square", format!("tl-palette-item-{}", shade.clone().to_string()))}
                />
                <span class={"tl-shade-label"}>{shade.to_string()}</span>
                {match theme.clone() {
                  ThemeColor::Custom(_) => {
                    html! {
                      <span class={"tl-shade-colour"}>{shades.get(&format!("--tl-primary-{}", shade.clone().to_string()))}</span>
                    }
                  }
                  _ => {
                    html! {
                      <span class={"tl-shade-colour"}>{format!("colors.{}.{}", theme.clone().to_string().to_lowercase(), shade)}</span>
                    }
                  }
                }

                }
              </div>
            })}
          </div>
        </div>
      </div>
    </div>
  }
}