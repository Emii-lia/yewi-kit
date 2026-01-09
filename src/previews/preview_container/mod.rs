use yew::{classes, function_component, html, AttrValue, Classes, Html, Properties};
use crate::components::{CodePreview, Tab, Tabs};
use crate::types::Color;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub title: String,
  #[prop_or_default]
  pub code: AttrValue,
  pub children: Html,
  #[prop_or_default]
  pub class: Classes
}

#[function_component(PreviewContainer)]
pub(crate) fn preview_container (Props {
  children,
  code,
  title,
  class
}: &Props) -> Html {
  html! {
    <div class="PreviewContainer">
      <Tabs color={Color::Dark}>
        <Tab label={html! { title.clone() }} value={"preview"}>
          <div class={classes!("preview-content", class.clone())}>
            { children.clone() }
          </div>
        </Tab>
        <Tab label={"Code"} value={"code"}>
          <CodePreview code={code.clone()}/>
        </Tab>
      </Tabs>
    </div>
  }
}