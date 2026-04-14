pub mod data;

use yew::{component, html, Html};
use yew_icons::IconData;
use crate::components::button::{Button, ButtonVariant};
use crate::components::code_preview::CodePreview;
use crate::components::sidebar::{Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupTitle, SidebarHeader, SidebarMenu, SidebarMenuItem, SidebarSubMenu, SidebarSubMenuContent, SidebarSubMenuTitle, SidebarTitle, SidebarTrigger};
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
        {"Sidebar"}
      </h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Add a sidebar to your application for navigation or additional content."}
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
                <SidebarMenuItem active=true icon={IconData::LUCIDE_USER}>
                  {"Avatar"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_INFO}>
                  {"Badge"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_MOUSE_POINTER_CLICK}>
                  {"Button"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_LAYERS}>
                  {"Carousel"}
                </SidebarMenuItem>
                <SidebarSubMenu>
                  <SidebarSubMenuTitle>
                    {"Layout"}
                  </SidebarSubMenuTitle>
                  <SidebarSubMenuContent>
                    <SidebarMenuItem icon={IconData::LUCIDE_PLUS}>
                      {"Collapse"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_RECTANGLE_HORIZONTAL}>
                      {"Modal"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_COMPONENT}>
                      {"Tabs"}
                    </SidebarMenuItem>
                  </SidebarSubMenuContent>
                </SidebarSubMenu>
                <SidebarSubMenu>
                  <SidebarSubMenuTitle>
                    {"Form"}
                  </SidebarSubMenuTitle>
                  <SidebarSubMenuContent>
                    <SidebarMenuItem icon={IconData::LUCIDE_CHECK_SQUARE}>
                      {"Checkbox"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_FILE_INPUT}>
                      {"File Input"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_FORM_INPUT}>
                      {"Input"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_KEY}>
                      {"PasswordInput"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_CHECK_CIRCLE}>
                      {"Radio"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_LIST_CHECKS}>
                      {"Select"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_TEXT_CURSOR_INPUT}>
                      {"Textarea"}
                    </SidebarMenuItem>
                  </SidebarSubMenuContent>
                </SidebarSubMenu>
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
                        <SidebarMenuItem active=true icon={IconData::LUCIDE_USER}>
                          {"Avatar"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_INFO}>
                          {"Badge"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_MOUSE_POINTER_CLICK}>
                          {"Button"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_LAYERS}>
                          {"Carousel"}
                        </SidebarMenuItem>
                        <SidebarSubMenu>
                          <SidebarSubMenuTitle>
                            {"Layout"}
                          </SidebarSubMenuTitle>
                          <SidebarSubMenuContent>
                            <SidebarMenuItem icon={IconData::LUCIDE_PLUS}>
                              {"Collapse"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_RECTANGLE_HORIZONTAL}>
                              {"Modal"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_COMPONENT}>
                              {"Tabs"}
                            </SidebarMenuItem>
                          </SidebarSubMenuContent>
                        </SidebarSubMenu>
                        <SidebarSubMenu>
                          <SidebarSubMenuTitle>
                            {"Form"}
                          </SidebarSubMenuTitle>
                          <SidebarSubMenuContent>
                            <SidebarMenuItem icon={IconData::LUCIDE_CHECK_SQUARE}>
                              {"Checkbox"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_FILE_INPUT}>
                              {"File Input"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_FORM_INPUT}>
                              {"Input"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_KEY}>
                              {"PasswordInput"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_CHECK_CIRCLE}>
                              {"Radio"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_LIST_CHECKS}>
                              {"Select"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_TEXT_CURSOR_INPUT}>
                              {"Textarea"}
                            </SidebarMenuItem>
                          </SidebarSubMenuContent>
                        </SidebarSubMenu>
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
                <SidebarMenuItem active=true icon={IconData::LUCIDE_USER}>
                  {"Avatar"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_INFO}>
                  {"Badge"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_MOUSE_POINTER_CLICK}>
                  {"Button"}
                </SidebarMenuItem>
                <SidebarMenuItem icon={IconData::LUCIDE_LAYERS}>
                  {"Carousel"}
                </SidebarMenuItem>
                <SidebarSubMenu>
                  <SidebarSubMenuTitle>
                    {"Layout"}
                  </SidebarSubMenuTitle>
                  <SidebarSubMenuContent>
                    <SidebarMenuItem icon={IconData::LUCIDE_PLUS}>
                      {"Collapse"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_RECTANGLE_HORIZONTAL}>
                      {"Modal"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_COMPONENT}>
                      {"Tabs"}
                    </SidebarMenuItem>
                  </SidebarSubMenuContent>
                </SidebarSubMenu>
                <SidebarSubMenu>
                  <SidebarSubMenuTitle>
                    {"Form"}
                  </SidebarSubMenuTitle>
                  <SidebarSubMenuContent>
                    <SidebarMenuItem icon={IconData::LUCIDE_CHECK_SQUARE}>
                      {"Checkbox"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_FILE_INPUT}>
                      {"File Input"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_FORM_INPUT}>
                      {"Input"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_KEY}>
                      {"PasswordInput"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_CHECK_CIRCLE}>
                      {"Radio"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_LIST_CHECKS}>
                      {"Select"}
                    </SidebarMenuItem>
                    <SidebarMenuItem icon={IconData::LUCIDE_TEXT_CURSOR_INPUT}>
                      {"Textarea"}
                    </SidebarMenuItem>
                  </SidebarSubMenuContent>
                </SidebarSubMenu>
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
                        <SidebarMenuItem active=true icon={IconData::LUCIDE_USER}>
                          {"Avatar"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_INFO}>
                          {"Badge"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_MOUSE_POINTER_CLICK}>
                          {"Button"}
                        </SidebarMenuItem>
                        <SidebarMenuItem icon={IconData::LUCIDE_LAYERS}>
                          {"Carousel"}
                        </SidebarMenuItem>
                        <SidebarSubMenu>
                          <SidebarSubMenuTitle>
                            {"Layout"}
                          </SidebarSubMenuTitle>
                          <SidebarSubMenuContent>
                            <SidebarMenuItem icon={IconData::LUCIDE_PLUS}>
                              {"Collapse"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_RECTANGLE_HORIZONTAL}>
                              {"Modal"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_COMPONENT}>
                              {"Tabs"}
                            </SidebarMenuItem>
                          </SidebarSubMenuContent>
                        </SidebarSubMenu>
                        <SidebarSubMenu>
                          <SidebarSubMenuTitle>
                            {"Form"}
                          </SidebarSubMenuTitle>
                          <SidebarSubMenuContent>
                            <SidebarMenuItem icon={IconData::LUCIDE_CHECK_SQUARE}>
                              {"Checkbox"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_FILE_INPUT}>
                              {"File Input"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_FORM_INPUT}>
                              {"Input"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_KEY}>
                              {"PasswordInput"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_CHECK_CIRCLE}>
                              {"Radio"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_LIST_CHECKS}>
                              {"Select"}
                            </SidebarMenuItem>
                            <SidebarMenuItem icon={IconData::LUCIDE_TEXT_CURSOR_INPUT}>
                              {"Textarea"}
                            </SidebarMenuItem>
                          </SidebarSubMenuContent>
                        </SidebarSubMenu>
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