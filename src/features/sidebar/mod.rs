mod hooks;
pub mod provider;
mod store;

mod components {
  pub mod nav_item;
}

use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::routes::AppRoute;
use crate::features::provider::SidebarContext;
use crate::features::sidebar::components::nav_item::NavItem;
use crate::features::sidebar::store::use_sidebar_store;

#[function_component(Sidebar)]
pub(crate) fn sidebar() -> Html {
  let SidebarContext { route_group, is_open, toggle_open } = use_sidebar_store();
  html! {
    <aside class={classes!(
      "Sidebar",
      is_open.then_some("sidebar-open")
    )}>
      <div class="sidebar-header">
        <div
          class={classes!("sidebar-header-content")}
        >
          <div
            onclick={toggle_open}
            title={"Toggle Sidebar"}
            class="sidebar-logo"
            style={"--logo: url('/icons/logo.png');"}
          />
          <Link<AppRoute>
            to={AppRoute::Home}
            classes="sidebar-title"
          >
            {"Yewi-kit"}
          </Link<AppRoute>>
        </div>
      </div>
      <div class="sidebar-menu">
        {for route_group.iter().map(|rs| {
          html! {
            <div class="sidebar-section">
              <h3 class="sidebar-section-title">{rs.title.clone()}</h3>
              <div class="sidebar-section-items">
                {for rs.routes.iter().map(|route| {
                  html! {
                    <NavItem
                      href={route.clone()}
                      label={route.clone().to_string()}
                      key={route.clone().to_string()}
                    />
                  }
                })}
              </div>
            </div>
          }
        })}
      </div>
    </aside>
  }
}