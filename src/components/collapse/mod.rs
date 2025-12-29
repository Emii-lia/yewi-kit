use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub summary: AttrValue,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes
}
#[function_component(Collapse)]
pub(crate) fn collapse(props: &Props) -> Html {
  html! {
    <div class="Collapse" tabindex={"1"}>
      <div class="collapse-trigger">{&props.summary}</div>
      <div class={classes!("collapse-content", &props.class)}>
        {props.children.clone()}
      </div>
    </div>
  }
}