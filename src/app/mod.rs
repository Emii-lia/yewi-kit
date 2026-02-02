mod button;
mod badge;
mod routes;
mod page;
mod not_found;
mod avatar;
mod input;
mod select;
mod checkbox;
mod modal;
mod textarea;
mod radio;
mod tabs;
mod avatar_group;
mod collapse;
mod file_input;
mod divider;
mod card;
mod password_input;
mod progress;
mod carousel;
mod dropdown;
mod toast;
mod table;
mod installation;

pub(crate) use button::*;
pub(crate) use badge::*;
pub(crate) use routes::*;
pub(crate) use page::*;
pub(crate) use input::*;
pub(crate) use select::*;
pub(crate) use checkbox::*;
pub(crate) use tabs::*;
pub(crate) use avatar::*;
pub(crate) use collapse::*;
pub(crate) use file_input::*;
pub(crate) use divider::*;
pub(crate) use avatar_group::*;
pub(crate) use not_found::*;
pub(crate) use modal::*;
pub(crate) use radio::*;
pub(crate) use textarea::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};
use crate::components::{Modal, ModalProvider, ToastContainer, ToastProvider};

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <ModalProvider>
        <ToastProvider>
          <div class="app">
            <ToastContainer/>
            <Modal/>
            <Switch<AppRoute> render={switch}/>
          </div>
        </ToastProvider>
      </ModalProvider>
    </BrowserRouter>
  }
}