use gloo::timers::callback::Timeout;
use yew::{hook, html, Callback, Children, Classes, use_state, use_effect_with};
use crate::components::toast::types::{ToastItem, ToastPosition, ToastType};


#[derive(Clone, PartialEq, Default)]
pub struct ToastState {
  pub position: Option<ToastPosition>,
  pub duration: Option<usize>,
  pub class: Option<Classes>
}

#[derive(Clone, PartialEq)]
pub(crate) struct ToastContext {
  pub default: Callback<(Children, Option<ToastState>)>,
  pub success: Callback<(Children, Option<ToastState>)>,
  pub error: Callback<(Children, Option<ToastState>)>,
  pub items: Vec<ToastItem>,
  pub on_remove: Callback<usize>
}

#[hook]
pub fn use_toast() -> ToastContext {
  let items = use_state(Vec::<ToastItem>::new);
  let next_id = use_state(|| 0usize);

  let on_remove = {
    let items = items.clone();
    Callback::from(move |ide: usize| {
      let mut new_items = (*items).clone();
      new_items.retain(|item| item.id != ide);
      items.set(new_items);
    })
  };

  let add = |variant: ToastType| {
    let items = items.clone();
    let next_id = next_id.clone();

    Callback::from(move |(children, state): (Children, Option<ToastState>)| {
      let state = state.unwrap_or_default();
      let position = state.position.unwrap_or(ToastPosition::BottomRight);
      let duration = state.duration.unwrap_or(3000);
      let class = state.class.unwrap_or_default();

      let id = *next_id;
      next_id.set(id + 1);

      let item = ToastItem {
        id,
        children,
        variant: variant.clone(),
        position,
        duration,
        class
      };

      let mut new_items = (*items).clone();
      new_items.push(item);
      items.set(new_items);
    })
  };

  let default = add(ToastType::Default);
  let success = add(ToastType::Success);
  let error = add(ToastType::Error);
  
  ToastContext {
    default,
    success,
    error,
    items: (*items).clone(),
    on_remove
  }
}


#[derive(Clone, PartialEq)]
pub(crate) struct HookParams {
  pub(crate) id: usize,
  pub(crate) duration: usize,
  pub(crate) on_close: Callback<()>
}

pub(crate) struct HookResponse {
  pub(crate) is_open: bool,
}
#[hook]
pub(crate) fn use_toast_item(params: HookParams) -> HookResponse {
  let HookParams { duration, on_close, id } = params;

  let is_open = use_state(|| true);

  {
    let is_open = is_open.clone();
    use_effect_with(id, move |_| {
      is_open.set(true);
      || ()
    });
  }

  {
    let is_open = is_open.clone();
    let on_close = on_close.clone();

    use_effect_with(
      (duration, id),
      move |(duration, _id)| {
        let mut timeout: Option<Timeout> = None;
        if *duration > 0 {
          let is_open = is_open.clone();
          let on_close = on_close.clone();

          timeout = Some(Timeout::new(*duration as u32, move || {
            is_open.set(false);
            on_close.emit(());
          }));
        }
        move || drop(timeout)
      }
    );
  }


  HookResponse {
    is_open: *is_open,
  }
}