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

pub(crate) use button::*;
pub(crate) use badge::*;
pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};
use crate::components::{Modal, ModalProvider};
use crate::features::Sidebar;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <ModalProvider>
        <div class="app">
          <Modal/>
          <Sidebar/>
          <Switch<AppRoute> render={switch}/>
        </div>
      </ModalProvider>
    </BrowserRouter>
  }
}