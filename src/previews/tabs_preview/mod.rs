mod data;

use yew::{component, html, Html};
use crate::components::avatar::Avatar;
use crate::components::badge::Badge;
use crate::components::button::Button;
use crate::components::code_preview::CodePreview;
use crate::components::tabs::{Tab, Tabs};
use crate::features::component_table::ComponentTable;
use crate::features::prop_table::PropTable;
use crate::previews::PreviewContainer;
use crate::previews::tabs_preview::data::{get_components, get_props};
use crate::types::{Color, Size};

#[component(TabsPreview)]
pub(crate) fn tabs_preview() -> Html {
  let components = get_components();
  let props = get_props();

  html! {
    <div class="PreviewSection">
      <h2 class="preview-title">
        {"Tabs"}
      </h2>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Installation"}
        </h2>
        <div class="preview-header-description">
          {"Organize content into separate views."}
        </div>
        <CodePreview code={"yewi add tabs"}/>
      </div>
      <div class="preview-subsection">
        <h2 class="preview-subsection-title">
          {"Examples"}
        </h2>
        <div class="preview-list">
          <PreviewContainer
            title={"Default"}
            code={r#"
      <Tabs>
        <Tab
          label={"Yewi kit"}
          value={"yewi-kit"}
        >
          <div class={"w-full text-slate-500 text-sm p-1"}>
            {"A Rust-first UI kit and CLI for building clean, scalable Yew frontends. Fast to start, easy to grow."}
          </div>
        </Tab>
        <Tab
          label={"Yewi CLI"}
          value={"yewi-cli"}
        >
          <div class="w-full text-slate-500 text-sm p-1">
            {"Yewi-cli is a command-line tool for managing UI components in your Yew projects, inspired by shadcn/ui.
              It allows you to create new projects from a pre-configured template, add components from the yewi-kit repository, and more.
              The CLI is written in Rust and uses yew for the frontend.
            "}
          </div>
        </Tab>
        <Tab
          label={"Yewi Template"}
          value={"yewi-template"}
        >
          <div class="w-full text-slate-500 text-sm p-1">
            {"Template for Yewi app when using yewi-cli [yewi new project] "}
          </div>
        </Tab>
      </Tabs>
            "#}
          >
            <Tabs>
              <Tab
                label={"Yewi kit"}
                value={"yewi-kit"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {"A Rust-first UI kit for building clean, scalable Yew frontends. Fast to start, easy to grow."}
                </div>
              </Tab>
              <Tab
                label={"Yewi CLI"}
                value={"yewi-cli"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {r#"Yewi-cli is a command-line tool for managing UI components in your Yew projects, inspired by shadcn/ui.
                    It allows you to create new projects from a pre-configured template, add components from the yewi-kit repository, and more.
                    The CLI is written in Rust and uses yew for the frontend.
                  "#}
                </div>
              </Tab>
              <Tab
                label={"Yewi template"}
                value={"yewi-template"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {"Template for Yewi app when using yewi-cli [yewi new project] "}
                </div>
              </Tab>
            </Tabs>
          </PreviewContainer>
          <PreviewContainer
            title={"Colour"}
            code={r#"
    <Tabs color={Color::Rose}>
      <Tab
        label={"Yewi kit"}
        value={"yewi-kit"}
      >
        <div class={"w-full text-slate-500 text-sm p-1"}>
          {"A Rust-first UI kit for building clean, scalable Yew frontends. Fast to start, easy to grow."}
        </div>
      </Tab>
      <Tab
        label={"Yewi CLI"}
        value={"yewi-cli"}
      >
        <div class="w-full text-slate-500 text-sm p-1">
          {"Yewi-cli is a command-line tool for managing UI components in your Yew projects, inspired by shadcn/ui.
            It allows you to create new projects from a pre-configured template, add components from the yewi-kit repository, and more.
            The CLI is written in Rust and uses yew for the frontend.
          "}
        </div>
      </Tab>
      <Tab
        label={"Yewi Template"}
        value={"yewi template"}
      >
        <div class="w-full text-slate-500 text-sm p-1">
          {"Template for Yewi app when using yewi-cli [yewi new project] "}
        </div>
      </Tab>
    </Tabs>
            "#}
          >
            <Tabs color={Color::Rose}>
              <Tab
                label={"Yewi kit"}
                value={"yewi-kit"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {"A Rust-first UI kit for building clean, scalable Yew frontends. Fast to start, easy to grow."}
                </div>
              </Tab>
              <Tab
                label={"Yewi CLI"}
                value={"yewi-cli"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {r#"Yewi-cli is a command-line tool for managing UI components in your Yew projects, inspired by shadcn/ui.
                    It allows you to create new projects from a pre-configured template, add components from the yewi-kit repository, and more.
                    The CLI is written in Rust and uses yew for the frontend.
                  "#}
                </div>
              </Tab>
              <Tab
                label={"Yewi Template"}
                value={"yewi-template"}
              >
                <div class="w-full text-slate-500 text-sm p-1">
                  {"Template for Yewi app when using yewi-cli [yewi new project] "}
                </div>
              </Tab>
            </Tabs>
          </PreviewContainer>
        </div>
      </div>
      <ComponentTable components={components} />
      <PropTable props={props} />
    </div>
  }
}