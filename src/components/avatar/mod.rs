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
  html! {
    if props.src != "" {
      <div
        class={classes!(
          "Avatar",
          "avatar-image",
          format!("{:?}", props.size.clone()).to_lowercase(),
          format!("{:?}", props.color.clone()).to_lowercase(),
          props.with_border.clone().then_some("with-border"),
          props.rounded.clone().then_some("rounded"),
          props.class.clone()
        )}
        style={format!("background-image: url('{}');", props.src.clone())}
        title={props.title.clone()}
      />
    } else {
      <div
        class={classes!(
          "Avatar",
          "avatar-initials",
          format!("{:?}", props.size.clone()).to_lowercase(),
          format!("{:?}", props.color.clone()).to_lowercase(),
          props.with_border.clone().then_some("with-border"),
          props.rounded.clone().then_some("rounded"),
        )}
        title={props.title.clone()}
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