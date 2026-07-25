use yew::{component, AttrValue, Children, Properties, Html, html, Classes, classes};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub title: AttrValue,
  #[prop_or_default]
  pub id: AttrValue,
  #[prop_or_default]
  pub class: Classes,
  pub children: Children
}

#[component(Section)]
pub fn section(props: &Props) -> Html {
  html! {
    <section
      class={classes!(
        "Section",
        &props.class
      )}
      id={&props.id}
    >
      <h2 class="section-title">{&props.title}</h2>
      {for props.children.iter()}
    </section>
  }
}