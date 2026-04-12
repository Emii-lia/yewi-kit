pub mod data;

use yew::{component, html, Html};
use yew_icons::IconData;
use crate::components::button::{Button, ButtonVariant};
use crate::components::code_preview::CodePreview;
use crate::components::sidebar::{Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupTitle, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarTitle, SidebarTrigger};
use crate::components::sidebar::provider::SidebarProvider;
use crate::components::sidebar::types::SidebarPosition;
use crate::features::component_table::ComponentTable;
use crate::features::prop_table::PropTable;
use crate::previews::PreviewContainer;
use crate::previews::sidebar_preview::data::{get_components, get_props};
use crate::types::Size;

#[component(SidebarPreview)]
pub fn sidebar_preview() -> Html {
  let components = get_components();
  let props = get_props();

  html! {
    <div  class="PreviewSection">
      <h2 class="preview-title">
        {"Table"}
      </h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {""}
        </div>
        <CodePreview code={"yewi add sidebar"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer title={"Left"} code={r#"
    <SidebarProvider >
      <Sidebar>
        <SidebarHeader>
          <SidebarTitle>
            {"Yewi-kit"}
          </SidebarTitle>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupTitle>
              {"Get Started"}
            </SidebarGroupTitle>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  {"Installation"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Quick Start"}
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupTitle>
              {"Components"}
            </SidebarGroupTitle>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem active=true>
                  {"Avatar"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Badge"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Button"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Checkbox"}
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>
          <Button
            variant={ButtonVariant::Tertiary}
            icon={IconData::LUCIDE_LOG_OUT}
            size={Size::Small}
          >
            {"Log Out"}
          </Button>
        </SidebarFooter>
      </Sidebar>
      <SidebarTrigger/>
    </SidebarProvider>
          "#}>
            <SidebarProvider>
              <Sidebar>
                <SidebarHeader>
                  <SidebarTitle>
                    {"Yewi-kit"}
                  </SidebarTitle>
                </SidebarHeader>
                <SidebarContent>
                  <SidebarGroup>
                    <SidebarGroupTitle>
                      {"Get Started"}
                    </SidebarGroupTitle>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        <SidebarMenuItem>
                          {"Installation"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Quick Start"}
                        </SidebarMenuItem>
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                  <SidebarGroup>
                    <SidebarGroupTitle>
                      {"Components"}
                    </SidebarGroupTitle>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        <SidebarMenuItem active=true>
                          {"Avatar"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Badge"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Button"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Checkbox"}
                        </SidebarMenuItem>
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                  <Button
                    variant={ButtonVariant::Tertiary}
                    icon={IconData::LUCIDE_LOG_OUT}
                    size={Size::Small}
                  >
                    {"Log Out"}
                  </Button>
                </SidebarFooter>
              </Sidebar>
              <SidebarTrigger/>
            </SidebarProvider>
          </PreviewContainer>
          <PreviewContainer title={"Right"} code={r#"
    <SidebarProvider>
      <Sidebar position={SidebarPosition::Right} >
        <SidebarHeader>
          <SidebarTitle>
            {"Yewi-kit"}
          </SidebarTitle>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupTitle>
              {"Get Started"}
            </SidebarGroupTitle>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  {"Installation"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Quick Start"}
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupTitle>
              {"Components"}
            </SidebarGroupTitle>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem active=true>
                  {"Avatar"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Badge"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Button"}
                </SidebarMenuItem>
                <SidebarMenuItem>
                  {"Checkbox"}
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>
          <Button
            variant={ButtonVariant::Tertiary}
            icon={IconData::LUCIDE_LOG_OUT}
            size={Size::Small}
          >
            {"Log Out"}
          </Button>
        </SidebarFooter>
      </Sidebar>
      <SidebarTrigger/>
    </SidebarProvider>
          "#}>
            <SidebarProvider>
              <Sidebar position={SidebarPosition::Right} >
                <SidebarHeader>
                  <SidebarTitle>
                    {"Yewi-kit"}
                  </SidebarTitle>
                </SidebarHeader>
                <SidebarContent>
                  <SidebarGroup>
                    <SidebarGroupTitle>
                      {"Get Started"}
                    </SidebarGroupTitle>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        <SidebarMenuItem>
                          {"Installation"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Quick Start"}
                        </SidebarMenuItem>
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                  <SidebarGroup>
                    <SidebarGroupTitle>
                      {"Components"}
                    </SidebarGroupTitle>
                    <SidebarGroupContent>
                      <SidebarMenu>
                        <SidebarMenuItem active=true>
                          {"Avatar"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Badge"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Button"}
                        </SidebarMenuItem>
                        <SidebarMenuItem>
                          {"Checkbox"}
                        </SidebarMenuItem>
                      </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                  <Button
                    variant={ButtonVariant::Tertiary}
                    icon={IconData::LUCIDE_LOG_OUT}
                    size={Size::Small}
                  >
                    {"Log Out"}
                  </Button>
                </SidebarFooter>
              </Sidebar>
              <SidebarTrigger/>
            </SidebarProvider>
          </PreviewContainer>

        </div>
      </div>
      <ComponentTable components={components}/>
      <PropTable props={props} />
    </div>
  }
}