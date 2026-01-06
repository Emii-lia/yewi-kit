mod types;

use yew::{classes, function_component, html, Classes, Html, Properties};
use crate::types::{Color, Size};
pub use types::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  #[prop_or_default]
  pub value: u32,
  #[prop_or(100)]
  pub max: u32,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(10)]
  pub height: u32,
  #[prop_or(Color::Blue)]
  pub color: Color,
  #[prop_or_default]
  pub show_percentage: bool,
  #[prop_or(ProgressVariant::Linear)]
  pub variant: ProgressVariant,
  #[prop_or(Size::Medium)]
  pub radial_size: Size,
  #[prop_or_default]
  pub as_fraction: bool,
}
#[function_component(Progress)]
pub(crate) fn progress(props: &Props) -> Html {
  let color_class = format!("{:?}", props.color).to_lowercase();
  let size_class = format!("{:?}", props.radial_size).to_lowercase();
  let progress_percentage = if props.max == 0 { 0.0 } else { (props.value.min(props.max) as f64 / props.max as f64) * 100.0 };
  let progress_value = if props.as_fraction {
    format!("{}/{}", props.value.min(props.max), props.max)
  } else {
    format!("{}%", progress_percentage.round())
  };

  match &props.variant {
    ProgressVariant::Linear => {

      html! {
        <div class="Progress">
          <div
            class={classes!(
              "progress-bar",
              color_class,
              &props.class
            )}
            style={format!("--value: {}%; height: {}px;",progress_percentage, props.height)}
          />
          {
            html! {
              if props.show_percentage {
                <span class="progress-value">
                  {progress_value.clone()}
                </span>
              }
            }
          }
        </div>
      }
    },
    ProgressVariant::Circular => {
      html! {
        <div
          class={classes!(
            "RadialProgress",
            size_class,
            props.show_percentage.then_some("show-percentage"),
            color_class,
            &props.class
          )}
          style={format!("--value: {}%;", progress_percentage)}
          data-progress={progress_value.clone()}
        />
      }
    }
  }
}