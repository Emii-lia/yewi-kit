use yew::{function_component, html, use_state, Callback, ContextProvider, Html};
use yew::html::ChildrenProps;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DropDownContextType {
  pub(crate) is_open: bool,
  pub(crate) toggle: Callback<()>,
  pub(crate) close: Callback<()>,
}

#[function_component(DropdownProvider)]
pub(crate) fn dropdown_provider(props: &ChildrenProps) -> Html {
  let is_open = use_state(|| false);
  
  let toggle = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      is_open.set(!*is_open);
    })
  };
  
  let close = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      is_open.set(false);
    })
  };
  
  let context = DropDownContextType {
    is_open: *is_open,
    toggle,
    close,
  };
  
  html! {
    <ContextProvider<DropDownContextType> context={context}>
      {props.children.clone()}
    </ContextProvider<DropDownContextType>>
  }
}