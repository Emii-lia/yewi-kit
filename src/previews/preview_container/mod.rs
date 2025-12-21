use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
  pub title: String,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes
}

#[function_component(PreviewContainer)]
pub(crate) fn preview_container (Props {
  children,
  title,
  class
}: &Props) -> Html {
  html! {
    <div class="PreviewContainer">
      <h1 class="preview-container-title">{title}</h1>
      <div class={classes!("preview-content", class)}>{children.clone()}</div>
    </div>
  }
}