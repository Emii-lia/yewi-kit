mod hooks;

mod components {
  pub mod nav_item;
}

use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::routes::AppRoute;
use crate::features::sidebar::components::nav_item::NavItem;
use crate::features::sidebar::hooks::use_sidebar;

#[function_component(Sidebar)]
pub(crate) fn sidebar() -> Html {
  let route_group = use_sidebar();
  html! {
    <aside class="Sidebar">
      <div class="sidebar-header">
        <Link<AppRoute>
          classes={classes!("sidebar-header-content")}
          to={AppRoute::Home}
        >
          <span class="sidebar-logo">{"Y"}</span>
          <span class="sidebar-title">{"Yewi-kit"}</span>
        </Link<AppRoute>>
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