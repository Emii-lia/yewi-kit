use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub vertical: bool,
  #[prop_or_default]
  pub class: Classes
}

#[function_component(Divider)]
pub(crate) fn divider(props: &Props) -> Html {
  let has_children = props.children.len() > 0;
  html! {
    <div class={classes!(
      "Divider",
      props.vertical.then_some("vertical"),
      has_children.then_some("has-children"),
      &props.class
    )}>
      {props.children.clone()}
    </div>
  }
}