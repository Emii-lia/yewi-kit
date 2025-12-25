use yew::{function_component, html, ContextProvider, Html};
use yew::html::ChildrenProps;
use crate::components::{use_modal, ModalHookResponse};

#[function_component(ModalProvider)]
pub(crate) fn modal_provider(props: &ChildrenProps) -> Html {
  let modal_vars = use_modal();
  html! {
    <ContextProvider<ModalHookResponse> context={modal_vars}>
      {props.children.clone()}
    </ContextProvider<ModalHookResponse>>
  }
}