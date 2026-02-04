use yew::{classes, function_component, html, Children, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ChildrenPropsWithClass {
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(HomeSection)]
pub fn home_section(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <div class={classes!("HomeSection", &props.class)}>
      {props.children.clone()}
    </div>
  }
}
#[function_component(HomeSectionText)]
pub fn home_section_text(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <div class={classes!("HomeSectionText", &props.class)}>
      {props.children.clone()}
    </div>
  }
}
#[function_component(HomeSectionContent)]
pub fn home_section_content(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <div class={classes!("HomeSectionContent", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[function_component(HomeSectionContainer)]
pub fn home_section_container(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <div class={classes!("HomeSectionContainer", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[function_component(HomeSectionTitle)]
pub fn home_section_title(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <h2 class={classes!("HomeSectionTitle", &props.class)}>
      {props.children.clone()}
    </h2>
  }
}

#[function_component(HomeSectionDescription)]
pub fn home_section_description(props: &ChildrenPropsWithClass) -> Html {
  html! {
    <p class={classes!("HomeSectionDescription", &props.class)}>
      {props.children.clone()}
    </p>
  }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HomeSectionIllustrationProps {
  #[prop_or_default]
  pub command: Option<String>,
  #[prop_or_default]
  pub output: Vec<String>,
  #[prop_or_default]
  pub class: Classes,
}
#[function_component(HomeSectionIllustration)]
pub fn home_section_illustration(props: &HomeSectionIllustrationProps) -> Html {
  html! {
    <div class={classes!("HomeSectionIllustration", &props.class)}>
      <div class="illustration-content">
        {html! {
          if let Some(command) = props.command.clone() {
            <div class="illustration-command">
              <span class="primary-command">{"$ "}</span> {command}
            </div>
          }
        }}
        <div class="illustration-output">
          {props.output.clone().into_iter().map(|output| html! {<pre>{output}</pre>}).collect::<Html>()}
        </div>
      </div>
    </div>
  }
}
