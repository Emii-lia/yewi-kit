mod button;
mod badge;
mod routes;
mod page;
mod not_found;

pub(crate) use button::*;
pub(crate) use badge::*;
pub(crate) use routes::*;
pub(crate) use page::*;

use yew::{function_component, html, Html};
use yew_router::prelude::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<AppRoute> render={switch}/>
    </BrowserRouter>
  }
}