use yew::{function_component, html, Html};
use crate::components::{Avatar, Badge, Button, CodePreview, Tab, Tabs};
use crate::previews::PreviewContainer;
use crate::types::{Color, Size};

#[function_component(TabsPreview)]
pub(crate) fn tabs_preview() -> Html {
  html! {
    <div class="PreviewSection">
      <h1 class="preview-title">
        {"Tabs"}
      </h1>
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
        label={"Button"}
        value={"button"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Button size={Size::Small}>
            {"Primary"}
          </Button>
          <Button size={Size::Medium}>
            {"Primary"}
          </Button>
          <Button size={Size::Large}>
            {"Primary"}
          </Button>
        </div>
      </Tab>
      <Tab
        label={"Label"}
        value={"label"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Badge label="Default" color={Color::Red}/>
          <Badge label="Default" color={Color::Blue}/>
          <Badge label="Default" color={Color::Green}/>
          <Badge label="Default" color={Color::Yellow}/>
        </div>
      </Tab>
      <Tab
        label={"Avatar"}
        value={"avatar"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Avatar alt={"John Doe"} size={Size::Small}/>
          <Avatar alt={"John Doe"} size={Size::Medium}/>
          <Avatar alt={"John Doe"} size={Size::Large}/>
        </div>
      </Tab>
    </Tabs>
            "#}
          >
            <Tabs>
              <Tab
                label={"Button"}
                value={"button"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Button size={Size::Small}>
                    {"Primary"}
                  </Button>
                  <Button size={Size::Medium}>
                    {"Primary"}
                  </Button>
                  <Button size={Size::Large}>
                    {"Primary"}
                  </Button>
                </div>
              </Tab>
              <Tab
                label={"Label"}
                value={"label"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Badge label="Default" color={Color::Red}/>
                  <Badge label="Default" color={Color::Blue}/>
                  <Badge label="Default" color={Color::Green}/>
                  <Badge label="Default" color={Color::Yellow}/>
                </div>
              </Tab>
              <Tab
                label={"Avatar"}
                value={"avatar"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Avatar alt={"John Doe"} size={Size::Small}/>
                  <Avatar alt={"John Doe"} size={Size::Medium}/>
                  <Avatar alt={"John Doe"} size={Size::Large}/>
                </div>
              </Tab>
            </Tabs>
          </PreviewContainer>
          <PreviewContainer
            title={"Colour"}
            code={r#"
    <Tabs color={Color::Rose}>
      <Tab
        label={"Button"}
        value={"button"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Button size={Size::Small}>
            {"Primary"}
          </Button>
          <Button size={Size::Medium}>
            {"Primary"}
          </Button>
          <Button size={Size::Large}>
            {"Primary"}
          </Button>
        </div>
      </Tab>
      <Tab
        label={"Label"}
        value={"label"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Badge label="Default" color={Color::Red}/>
          <Badge label="Default" color={Color::Blue}/>
          <Badge label="Default" color={Color::Green}/>
          <Badge label="Default" color={Color::Yellow}/>
        </div>
      </Tab>
      <Tab
        label={"Avatar"}
        value={"avatar"}
      >
        <div class="w-full flex items-center justify-center gap-5">
          <Avatar alt={"John Doe"} size={Size::Small}/>
          <Avatar alt={"John Doe"} size={Size::Medium}/>
          <Avatar alt={"John Doe"} size={Size::Large}/>
        </div>
      </Tab>
    </Tabs>
            "#}
          >
            <Tabs color={Color::Rose}>
              <Tab
                label={"Button"}
                value={"button"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Button size={Size::Small}>
                    {"Primary"}
                  </Button>
                  <Button size={Size::Medium}>
                    {"Primary"}
                  </Button>
                  <Button size={Size::Large}>
                    {"Primary"}
                  </Button>
                </div>
              </Tab>
              <Tab
                label={"Label"}
                value={"label"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Badge label="Default" color={Color::Red}/>
                  <Badge label="Default" color={Color::Blue}/>
                  <Badge label="Default" color={Color::Green}/>
                  <Badge label="Default" color={Color::Yellow}/>
                </div>
              </Tab>
              <Tab
                label={"Avatar"}
                value={"avatar"}
              >
                <div class="w-full flex items-center justify-center gap-5">
                  <Avatar alt={"John Doe"} size={Size::Small}/>
                  <Avatar alt={"John Doe"} size={Size::Medium}/>
                  <Avatar alt={"John Doe"} size={Size::Large}/>
                </div>
              </Tab>
            </Tabs>
          </PreviewContainer>
        </div>
      </div>
    </div>
  }
}