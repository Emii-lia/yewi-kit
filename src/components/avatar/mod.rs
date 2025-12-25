use yew::{classes, function_component, html, AttrValue, Html, Properties};
use crate::types::{Color, Size};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
  #[prop_or_default]
  pub alt: AttrValue,
  #[prop_or_default]
  pub src: AttrValue,
  #[prop_or_default]
  pub class: String,
  #[prop_or_default]
  pub rounded: bool,
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or_default]
  pub with_border: bool,
  #[prop_or_default]
  pub title: AttrValue,
  #[prop_or(Color::Blue)]
  pub color: Color,
}

#[function_component(Avatar)]
pub(crate) fn avatar(props: &Props) -> Html {
  let size_class = format!("{:?}", &props.size).to_lowercase();
  let color_class = format!("{:?}", &props.color).to_lowercase();
  let with_border_class = props.with_border.then_some("with-border");
  let rounded_class = props.rounded.then_some("rounded");
  
  html! {
    if props.src != "" {
      <div
        class={classes!(
          "Avatar",
          "avatar-image",
          size_class,
          color_class,
          with_border_class,
          rounded_class,
          &props.class
        )}
        style={format!("background-image: url('{}');", &props.src)}
        title={&props.title}
      />
    } else {
      <div
        class={classes!(
          "Avatar",
          "avatar-initials",
          size_class,
          color_class,
          with_border_class,
          rounded_class,
        )}
        title={&props.title}
      >
        {if props.alt.clone() != "" {
          let initials: String = props.alt
            .split_whitespace()
            .filter_map(|s| s.chars().next())
            .take(2)
            .collect();
          initials
        } else {
          "U".to_string()
        }}
      </div>
    }
  }
}