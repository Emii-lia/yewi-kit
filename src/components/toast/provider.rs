use yew::{component, html, ContextProvider, Html};
use yew::html::ChildrenProps;
use crate::components::toast::hooks::ToastContext;
use crate::components::use_toast;

#[component(ToastProvider)]
pub(crate) fn toast_provider(props: &ChildrenProps) -> Html {
  let toast = use_toast();
  html! {
    <ContextProvider<ToastContext> context={toast}>
      {props.children.clone()}
    </ContextProvider<ToastContext>>
  }
}