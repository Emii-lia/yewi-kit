use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::window;
use yew::{hook, use_effect_with, use_state, Callback};
use crate::components::sidebar::provider::SidebarContextType;
use crate::components::sidebar::types::SidebarState;

#[hook]
pub fn use_is_mobile(
  breakpoint: Option<u32>,
) -> bool {
  let breakpoint = breakpoint.unwrap_or(768);
  let is_mobile = use_state(|| false);

  {
    let is_mobile = is_mobile.clone();
    use_effect_with(breakpoint, move |breakpoint| {
      let is_mobile = is_mobile.clone();
      let media_query = format!("(max-width: {}px)", breakpoint);

      let media_query_list = window()
        .unwrap()
        .match_media(&media_query)
        .unwrap()
        .unwrap();

      let update_is_mobile = {
        let is_mobile = is_mobile.clone();
        let media_query_list = media_query_list.clone();
        move || {
          is_mobile.set(media_query_list.matches());
        }
      };

      update_is_mobile();

      let closure = Closure::wrap(Box::new(update_is_mobile) as Box<dyn FnMut()>);
      let _ = media_query_list.add_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));

      move || {
        let _ = media_query_list.remove_listener_with_opt_callback(Some(closure.as_ref().unchecked_ref()));
        drop(closure);
      }
    })
  };

  (*is_mobile).clone()
}

#[derive(Debug, PartialEq)]
pub struct SidebarProviderHookParams {
  pub default_open: bool,
  pub open: Option<bool>,
  pub onopenchange: Option<Callback<bool>>,
}

#[hook]
pub fn use_sidebar_provider(params: SidebarProviderHookParams) -> SidebarContextType {
  let is_mobile = use_is_mobile(None);
  let open_mobile = use_state(|| false);
  let open = use_state(|| params.default_open);
  let is_open = params.open.unwrap_or_else(|| *open);

  let set_open = {
    let onopenchange = params.onopenchange.clone();
    let open = open.clone();
    Callback::from(move |new_open: bool| {
      let open = open.clone();
      let onopenchange = onopenchange.clone();
      open.set(new_open);
      if let Some(onopenchange) = onopenchange {
        onopenchange.emit(new_open);
      }
    })
  };

  let toggle_sidebar = {
    let open = open.clone();
    let open_mobile = open_mobile.clone();
    Callback::from(move |_| {
      let open = open.clone();
      let open_mobile = open_mobile.clone();
      if is_mobile {
        open_mobile.set(!*open_mobile);
      } else {
        open.set(!*open);
      }
    })
  };

  let set_open_mobile = {
    let open_mobile = open_mobile.clone();
    Callback::from(move |new_open: bool| {
      let open_mobile = open_mobile.clone();
      open_mobile.set(new_open);
    })
  };

  let state = if is_open { SidebarState::Expanded } else { SidebarState::Collapsed };

  SidebarContextType {
    state,
    open: is_open,
    set_open,
    open_mobile: (*open_mobile).clone(),
    set_open_mobile,
    is_mobile,
    toggle_sidebar,
  }
}