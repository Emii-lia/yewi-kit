use yew::{component, html, Callback, ContextProvider, Html};
use yew::html::ChildrenProps;
use crate::features::theme_lab::hooks::use_theme_lab_provider;
use crate::types::theme::ThemeColor;

#[derive(Clone, PartialEq, Debug)]
pub struct ThemeLabContextType {
  pub theme: ThemeColor,
  pub set_theme: Callback<ThemeColor>,
}

#[component(ThemeLabProvider)]
pub fn theme_lab_provider(props: &ChildrenProps) -> Html {
  let ctx = use_theme_lab_provider();
  
  html! {
    <ContextProvider<ThemeLabContextType> context={ctx.clone()}>
      {props.children.clone()}
    </ContextProvider<ThemeLabContextType>>
  }
}