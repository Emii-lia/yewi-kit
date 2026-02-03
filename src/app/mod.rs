pub mod page;
pub mod not_found;
pub mod docs;
pub mod routes;


use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};
use crate::app::routes::{switch_main, AppRoute};
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
            <Switch<AppRoute> render={switch_main}/>
          </div>
        </ToastProvider>
      </ModalProvider>
    </BrowserRouter>
  }
}