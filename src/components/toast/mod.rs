use yew::{classes, function_component, html, Callback, Children, Classes, Html, Properties};
use crate::components::toast::hooks::{use_toast_item, HookParams, HookResponse};

mod types;
mod hooks;
mod provider;
mod store;

pub use hooks::{use_toast, ToastState};
pub use types::{ ToastPosition, ToastType};
pub(crate) use provider::ToastProvider;
pub(crate) use store::use_toast_store;

#[derive(Properties, Clone, PartialEq)]
pub(crate) struct ToastProps {
  pub id: usize,
  pub children: Children,
  #[prop_or(ToastPosition::BottomRight)]
  pub position: ToastPosition,
  #[prop_or(ToastType::Default)]
  pub variant: ToastType,
  #[prop_or(3000)]
  pub duration: usize,
  #[prop_or_default]
  pub class: Classes,
  pub on_close: Callback<()>
}

#[function_component(ToastContainer)]
pub(crate) fn toast_container() -> Html {
  let ctx = use_toast_store();
  let stack = |pos: ToastPosition| {
    let stack_class = format!("toast-{:?}", pos).to_lowercase();

    html! {
      <div class={classes!("ToastStack", &stack_class)}>
        {for ctx.items.iter().filter(|item| item.position == pos).map(|item| {
          let onclose = ctx.on_remove.clone();
          let id = item.id;

          html! {
            <Toast
              key={format!("{}-toast-{}", &stack_class, id)}
              id={item.id}
              position={item.position.clone()}
              variant={item.variant.clone()}
              duration={item.duration}
              class={item.class.clone()}
              on_close={Callback::from(move |_| onclose.emit(id))}
            >
              {item.children.clone()}
            </Toast>
          }
        })}
      </div>
    }
  };
  html! {
    <div class="ToastContainer" id="yewi-toast-container">
      {stack(ToastPosition::BottomLeft)}
      {stack(ToastPosition::BottomRight)}
      {stack(ToastPosition::BottomCenter)}
      {stack(ToastPosition::TopLeft)}
      {stack(ToastPosition::TopRight)}
      {stack(ToastPosition::TopCenter)}
    </div>
  }
}

#[function_component(Toast)]
pub(crate) fn toast(props: &ToastProps) -> Html {
  let position = format!("toast-{:?}", &props.position).to_lowercase();
  let variant = format!("toast-{:?}", &props.variant).to_lowercase();

  let HookResponse {
    is_open
  } = use_toast_item(HookParams {
    duration: props.duration,
    on_close: props.on_close.clone(),
    id: props.id
  });

  html! {
    <div class={classes!(
      "Toast",
      position,
      is_open.then_some("toast-open"),
      &props.class
    )}>
      <div class={classes!("Toast__content", variant)}>
        {props.children.clone()}
      </div>
      <button class="Toast__close" onclick={{
        let onclose = props.on_close.clone();
        Callback::from(move |_| onclose.emit(()))
      }}>
        {"x"}
      </button>
    </div>
  }
}