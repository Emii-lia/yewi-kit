pub mod hooks;

use yew::{component, html, Callback, Html};
use yew_router::prelude::Link;
use crate::app::routes::AppRoute;
use crate::components::sidebar::{Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupTitle, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarSubMenu, SidebarSubMenuContent, SidebarSubMenuTitle};
use crate::components::sidebar::types::SidebarPosition;
use crate::features::yewi_sidebar::hooks::use_yewi_sidebar;

#[component(YewiSidebar)]
pub fn yewi_sidebar() -> Html {
  let (get_stated, components, on_navigate, is_active) = use_yewi_sidebar();
  html! {
    <Sidebar class={"YewiSidebar"} position={SidebarPosition::Left}>
      <SidebarHeader class={"sidebar-header"}>
        <div
          class="sidebar-logo"
          style={"--logo: url('/icons/logo.png');"}
        />
        <Link<AppRoute>
          to={AppRoute::Home}
          classes="sidebar-title"
        >
          {"Yewi-kit"}
        </Link<AppRoute>>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupTitle>
            {get_stated.0.clone()}
          </SidebarGroupTitle>
          <SidebarGroupContent>
            <SidebarMenu>
              {for get_stated.1.iter().map(|nav| {
                html! {
                  <SidebarMenuItem
                    icon={nav.icon.clone()}
                    onclick={{
                      let on_navigate = on_navigate.clone();
                      let route = nav.route.clone();
                      Callback::from(move |e| {
                        on_navigate.emit((e, route.clone()));
                      })
                    }}
                    active={{
                      let is_active = is_active.clone();
                      let route = nav.route.clone();
                      is_active.emit(route.clone())
                    }}
                  >
                    {nav.route.clone().to_string()}
                  </SidebarMenuItem>
                }
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
        <SidebarGroup>
          <SidebarGroupTitle>
            {"Components"}
          </SidebarGroupTitle>
          <SidebarGroupContent>
            <SidebarMenu>
              {for components.iter().map(|(sub, items)| {
                html! {
                  <SidebarSubMenu>
                    <SidebarSubMenuTitle>
                      {sub.clone()}
                    </SidebarSubMenuTitle>
                    <SidebarSubMenuContent>
                      {for items.iter().map(|item| {
                        html! {
                          <SidebarMenuItem
                            icon={item.icon.clone()}
                            onclick={{
                              let on_navigate = on_navigate.clone();
                              let route = item.route.clone();
                              Callback::from(move |e| {
                                on_navigate.emit((e, route.clone()));
                              })
                            }}
                            active={{
                              let is_active = is_active.clone();
                              let route = item.route.clone();
                              is_active.emit(route.clone())
                            }}
                          >
                            {item.route.clone().to_string()}
                          </SidebarMenuItem>
                        }
                      })}
                    </SidebarSubMenuContent>
                  </SidebarSubMenu>
                }
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  }
}