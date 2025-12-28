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
  #[prop_or(2)]
  pub take: usize,
  #[prop_or_default]
  pub no_split: bool,
  #[prop_or_default]
  pub style: String,
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
        style={format!("background-image: url('{}'); {}", &props.src, &props.style)}
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
          &props.class
        )}
        title={&props.title}
        style={props.style.to_string()}
      >
        {if props.alt.clone() != "" {
          let initials: String = props.alt
            .split_whitespace()
            .filter_map(|s| s.chars().next())
            .take(props.take)
            .collect();
          if props.no_split { props.alt.to_string() } else { initials }
        } else {
          "U".to_string()
        }}
      </div>
    }
  }
}