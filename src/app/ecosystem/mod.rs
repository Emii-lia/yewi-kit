use yew::{html, Html};
use yew_router::prelude::Redirect;
use crate::app::ecosystem::route::EcosystemRoute;
use crate::app::ecosystem::yewi_cli::YewiCliPage;
use crate::app::ecosystem::yewi_seo::YewiSeoPage;
use crate::app::routes::AppRoute;
use crate::components::sidebar::provider::SidebarProvider;
use crate::components::sidebar::SidebarTrigger;
use crate::features::yewi_sidebar::YewiSidebar;

pub mod route;
pub mod page;
pub mod yewi_cli;
pub mod yewi_seo;

pub fn switch_ecosystem(route: EcosystemRoute) -> Html {
  let current_route = match route {
    EcosystemRoute::YewiCli => { html! { <YewiCliPage/> } }
    EcosystemRoute::YewiSeo => { html! { <YewiSeoPage/> } }
    EcosystemRoute::NotFound => { html! { <Redirect<AppRoute> to={AppRoute::NotFound} /> }}
  };

  html! {
    <div class="ecosystem-app">
      <SidebarProvider>
        <YewiSidebar/>
        <SidebarTrigger/>
        {current_route}
      </SidebarProvider>
    </div>
  }
}